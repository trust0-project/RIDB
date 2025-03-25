use js_sys::{Array, Object, Promise, Reflect};
use pool::POOL;
use utils::{can_use_single_index_lookup, create_database, cursor_fetch_and_filter, idb_request_result};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen_futures::JsFuture;
use crate::query::Query;
use crate::storage::internals::base_storage::BaseStorage;
use crate::storage::internals::core::CoreStorage;
use crate::operation::{OpType, Operation};
use web_sys::{IdbDatabase, IdbObjectStore};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::error::RIDBError;
use crate::query::options::QueryOptions;
use super::base::Storage;

pub mod utils;
pub mod pool;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents an IndexDB storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
export class IndexDB<T extends SchemaTypeRecord> extends BaseStorage<T> {
    /**
     * Frees the resources used by the in-memory storage.
     */
    free(): void;

    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
    ): Promise<
        IndexDB<
            SchemasCreate
        >
    >;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct IndexDB {
    core: CoreStorage,
    base: BaseStorage,
    db: Arc<Mutex<IdbDatabase>>,
    _error_handler: Arc<Mutex<Option<Closure<dyn FnMut(web_sys::Event)>>>>,
    _success_handler: Arc<Mutex<Option<Closure<dyn FnMut(web_sys::Event)>>>>,
}



impl Storage for IndexDB {
    async fn write(&self, op: Operation) -> Result<JsValue, RIDBError> {
        let store_name = &op.collection;
        let store = self.get_store(store_name)?;
        let schemas = self.base.schemas.borrow();
        let schema = schemas.get(op.collection.as_str())
            .ok_or_else(|| RIDBError::from("Collection not found"))?;

        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                let document = op.data.clone();

                // Extract primary key
                let primary_key = schema.primary_key.clone();
                let pk_value =  Reflect::get(&document, &JsValue::from_str(&primary_key))?;

                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(RIDBError::from("Document must contain a primary key"));
                }
                // Store the document and wait for completion
                let request = store.put_with_key(&document, &pk_value)?;
                idb_request_result(request).await?;
                Ok(document.clone())
            },
            OpType::DELETE => {
                let pk_value = op.data.clone();
                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(RIDBError::from("Primary key value is required for delete operation"));
                }

                // Delete the document using the primary key
                let request = store.delete(&pk_value)?;
                idb_request_result(request).await?;

                Ok(JsValue::from_str("Document deleted"))
            },
            _ => Err(RIDBError::from("Unsupported operation type")),
        }
    }

    async fn find(&self, collection_name: &str, query: Query, options: QueryOptions) -> Result<JsValue, RIDBError> {
        // Logger::debug(
        //     "IndexDB-Find",
        //     &JsValue::from(format!("Find method {}", collection_name)),
        // );

        let filtered_docs = self
            .collect_documents_for_query(collection_name, query, options)
            .await?;
        Ok(filtered_docs.into())
    }

    async fn find_document_by_id(&self, collection_name: &str, primary_key_value: JsValue) -> Result<JsValue, RIDBError> {
        let store_name = collection_name;
        if primary_key_value.is_undefined() || primary_key_value.is_null() {
            return Err(RIDBError::from("Primary key value is required"));
        }

        let store = self.get_store(store_name)?;

        // Logger::debug("IndexDB-Find-By-Id", &JsValue::from(&format!("Finding document with primary key: {:?}", primary_key_value)));

        let request = store.get(&primary_key_value)?;

        let result = idb_request_result(request).await?;

        if result.is_undefined() || result.is_null() {
            // Logger::debug("IndexDB-Find-By-Id",&JsValue::from("Document not found"));
            Ok(JsValue::null())
        } else {
            // Logger::debug("IndexDB-Find-By-Id",&JsValue::from("Document found"));
            Ok(result)
        }
    }

    async fn count(&self, collection_name: &str, query: Query, options: QueryOptions) -> Result<JsValue, RIDBError> {
        // Logger::debug(
        //     "IndexDB-Count",
        //     &JsValue::from(format!("Count method {}", collection_name)),
        // );
        let filtered_docs = self
            .collect_documents_for_query(collection_name, query, options)
            .await?;
        Ok(JsValue::from_f64(filtered_docs.length() as f64))
    }
    async fn close(&self) -> Result<JsValue, RIDBError> {
        // First, extract the name so we can remove from pool later
        let db_name = self.base.name.clone();
        
        // Create a list of store names before locking the database
        let stores: Vec<String>;
        {
            // Minimize the time the lock is held by using a scoped block
            let db = self.db.lock();
            stores = (0..db.object_store_names().length())
                .filter_map(|i| {
                    db.object_store_names().get(i).map(|name| name.as_str().to_string())
                })
                .collect();
        }

        // Create a read transaction for each store to ensure all operations are complete
        for store_name in stores {
            let transaction;
            {
                let db = self.db.lock();
                transaction = db.transaction_with_str(&store_name)?;
            }

            // Wait for the transaction to complete
            let promise = Promise::new(&mut |resolve, reject| {
                let oncomplete = Closure::once(Box::new(move |_: web_sys::Event| {
                    resolve.call0(&JsValue::undefined()).unwrap();
                }));

                let onerror = Closure::once(Box::new(move |e| {
                    reject.call1(&JsValue::undefined(), &e).unwrap();
                }));

                transaction.set_oncomplete(Some(oncomplete.as_ref().unchecked_ref()));
                transaction.set_onerror(Some(onerror.as_ref().unchecked_ref()));

                oncomplete.forget();
                onerror.forget();
            });

            JsFuture::from(promise).await?;
        }

        // Remove the connection from the pool first
        POOL.remove_connection(&db_name);

        // Now close the database
        {
            let db = self.db.lock();
            db.close();
        }
        
        Ok(JsValue::from_str("IndexDB database closed"))
    }

    async fn start(&self) -> Result<JsValue, RIDBError> {
        // Save the database name before testing the connection
        let db_name = self.base.name.clone();
        
        // Test if database is closed by attempting a simple transaction
        let db_is_closed = {
            // Keep lock scope as small as possible
            let db_guard = self.db.lock();
            let store_names = db_guard.object_store_names();
            
            if store_names.length() == 0 {
                true // No stores, likely closed or connection issue
            } else {
                let test_store = store_names.get(0).unwrap();
                db_guard.transaction_with_str(&test_store).is_err()
            }
        };
        
        if db_is_closed {
            // Database is closed, we need to reopen it
            //Logger::debug("IndexDB-Start", &JsValue::from_str("Reopening closed database connection"));
            
            // Clone the schemas for create_database
            let schemas_clone = self.base.schemas.borrow().clone();
            
            // Create a new database connection
            let new_db = create_database(&db_name, schemas_clone).await?;
            
            // Update the pool with new connection
            POOL.store_connection(db_name, Arc::downgrade(&new_db));
            
            // Update our internal database reference
            {
                let mut db_guard = self.db.lock();
                *db_guard = (*new_db).clone();
            }
            
            Ok(JsValue::from_str("IndexDB database started"))
        } else {
            //Logger::debug("IndexDB-Start", &JsValue::from_str("Database already started"));
            Ok(JsValue::from_str("IndexDB database already started"))
        }
    }
}




#[wasm_bindgen]
impl IndexDB {
    /// A helper function to collect and filter documents for a given query,
    /// respecting offsets and limits to avoid wasting time and resources.
    async fn collect_documents_for_query(
        &self,
        collection_name: &str,
        query: Query,
        options: QueryOptions
    ) -> Result<Array, RIDBError> {
        // Acquire references to the object store and schema
        let store = self.get_store(collection_name)?;
        let schemas = self.base.schemas.borrow();
        let schema = schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?
            .clone();

        // Attempt to figure out if we can leverage a single index
        let index_name_option = can_use_single_index_lookup(query.clone(), schema)?;

        // Determine offset and limit
        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(u32::MAX);

        // Clone the query data for filtering
        let core = self.core.clone();
        let normalized_query = query.clone().parse()?;
        // Build a "value_query" for final in-memory filter

        // Prepare the final, filtered documents array
        // but efficiently fetch them using a cursor approach.
        let documents = if let Some(index_name) = index_name_option {
            // There's a single suitable index
            let index_value = query.get(index_name.as_str())?;
            if let Ok(index) = store.index(&index_name) {
                // If "index_value" is an array of keys, we merge from multiple cursors
                if Array::is_array(&index_value) {
                    let key_array = Array::from(&index_value);
                    let merged_docs = Array::new();
                    for i in 0..key_array.length() {
                        let value_query = Query::new(normalized_query.clone(), query.clone().schema.clone())?;
                        let key = key_array.get(i);
                        let partial_result = cursor_fetch_and_filter(
                            Some(&index),
                            None,
                            &key,
                            core,
                            value_query,
                            offset,
                            limit,
                        ).await?;
                        // Merge partial_result into merged_docs
                        for j in 0..partial_result.length() {
                            merged_docs.push(&partial_result.get(j));
                        }
                    }
                    merged_docs
                } else {
                    let value_query = Query::new(normalized_query.clone(), query.clone().schema.clone())?;
                    // Single key fetch from this index
                    cursor_fetch_and_filter(
                        Some(&index),
                        None,
                        &index_value,
                        core,
                        value_query,
                        offset,
                        limit,
                    )
                    .await?
                }
            } else {
                let value_query = Query::new(normalized_query.clone(), query.clone().schema.clone())?;
                // If we couldn't get the index, do a cursor fetch for the entire store
                cursor_fetch_and_filter(
                    None,
                    Some(&store),
                    &JsValue::undefined(),
                    core,
                    value_query,
                    offset,
                    limit,
                )
                .await?
            }
        } else {
            let value_query = Query::new(normalized_query.clone(), query.clone().schema.clone())?;
            // No single index is usable; fetch everything via cursor on the store
            cursor_fetch_and_filter(
                None,
                Some(&store),
                &JsValue::undefined(),
                core,
                value_query,
                offset,
                limit,
            )
            .await?
        };

        Ok(documents)
    }

    /// Fetch documents by opening an IndexedDB cursor (on an index or store),
    /// then apply inline filtering and offset/limit constraints.

    pub fn get_stores(&self) -> Vec<String> {
        let store_names = self.db.lock().object_store_names();
        let stores: Vec<String> = (0..store_names.length())
            .filter_map(|i| {
                let store = store_names.get(i)?;
                Some(store.as_str().to_string())
            })
            .collect();
        stores
    }
    pub fn get_store(&self, store_name: &str) -> Result<IdbObjectStore, RIDBError>{
        let stores = self.get_stores();
        let transaction = match self.db.lock().transaction_with_str_and_mode(
            store_name,
            web_sys::IdbTransactionMode::Readwrite,
        ) {
            Ok(t) => t,
            Err(_e) => {
                return Err(RIDBError::from(&format!(
                    "Failed to access store '{}'. Available stores: {:?}",
                    store_name, stores
                )));
            }
        };

        let store = transaction.object_store(store_name)
            .map_err(|e| RIDBError::from(e));

        store
    }
    #[wasm_bindgen]
    pub async fn create(name: &str, schemas_js: Object) -> Result<IndexDB, RIDBError> {
        // Create the base storage with the provided schemas
        let base = BaseStorage::new(
            name.to_string(),
            schemas_js.clone(),
            None
        )?;

        // Clone the schemas for create_database
        let schemas_clone = base.schemas.borrow().clone();
        
        // Try to get an existing connection from the pool
        let db = match POOL.get_connection(name) {
            Some(db) => {
                //Logger::debug("IndexDB-Create", &JsValue::from_str("Reusing existing database connection"));
                db
            },
            None => {
                // Create new connection if none exists
                //Logger::debug("IndexDB-Create", &JsValue::from_str("Creating new database connection"));
                let db = create_database(name, schemas_clone).await?;
                
                // Store a weak reference in the pool to avoid circular references
                POOL.store_connection(name.to_string(), Arc::downgrade(&db));
                db
            }
        };
        
        // Wrap the database in an Arc<Mutex<>> for thread-safe access
        let db_mutex = Arc::new(Mutex::new((*db).clone()));
        
        // Create the storage instance
        Ok(IndexDB {
            base,
            core: CoreStorage {},
            db: db_mutex,
            _error_handler: Arc::new(Mutex::new(None)),
            _success_handler: Arc::new(Mutex::new(None)),
        })
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&self, op: Operation) -> Result<JsValue, RIDBError> {
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "find")]
    pub async fn find_js(&self, collection_name: &str, query: JsValue, options: QueryOptions) -> Result<JsValue, RIDBError> {
        let schemas = self.base.schemas.borrow();
        let schema = schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let query = Query::new(query, schema.clone())?;
        self.find(collection_name, query, options)
            .await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, RIDBError> {
        self.find_document_by_id(collection_name, primary_key).await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self, collection_name: &str, query: JsValue, options: QueryOptions) -> Result<JsValue, RIDBError> {
        let clone = self.clone();
        let schemas = clone.base.schemas.borrow();
        let schema = schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let query = Query::new(query.clone(), schema.clone())?;
        self.count(collection_name, query, options).await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&self) -> Result<JsValue, RIDBError> {
        self.close().await
    }

    #[wasm_bindgen(js_name = "start")]
    pub async fn start_js(&self) -> Result<JsValue, RIDBError> {
        self.start().await
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use wasm_bindgen_test::*;

    #[cfg(feature = "browser")]
    wasm_bindgen_test_configure!(run_in_browser);

    fn json_str_to_js_value(json_str: &str) -> Result<JsValue, JsValue> {
        let json_value: Value =
            serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(value_to_js_value(&json_value))
    }

    fn value_to_js_value(value: &Value) -> JsValue {
        match value {
            Value::Null => JsValue::null(),
            Value::Bool(b) => JsValue::from_bool(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    JsValue::from_f64(i as f64)
                } else if let Some(f) = n.as_f64() {
                    JsValue::from_f64(f)
                } else {
                    JsValue::undefined()
                }
            }
            Value::String(s) => JsValue::from_str(s),
            Value::Array(arr) => {
                let js_array = Array::new();
                for item in arr {
                    js_array.push(&value_to_js_value(item));
                }
                js_array.into()
            }
            Value::Object(obj) => {
                let js_obj = Object::new();
                for (key, value) in obj {
                    js_sys::Reflect::set(
                        &js_obj,
                        &JsValue::from_str(key),
                        &value_to_js_value(value),
                    )
                        .unwrap();
                }
                js_obj.into()
            }
        }
    }

    #[wasm_bindgen_test(async)]
    async fn test_empty_indexdb_storage() {
        let schemas_obj = Object::new();
        let schema_str = "{ \"version\": 1, \"primaryKey\": \"id\", \"type\": \"object\", \"properties\": { \"id\": { \"type\": \"string\", \"maxLength\": 60 } } }";
        let schema = json_str_to_js_value(schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();

        let db = IndexDB::create("test_db", schemas_obj).await;
        assert!(db.is_ok());

        // Clean up
        db.unwrap().close().await.unwrap();
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_create_operation() {
        let schemas_obj = Object::new();
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "required": ["id", "name"],
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();

        let  db = IndexDB::create("test_db_create", schemas_obj).await.unwrap();

        // Create a new item
        let new_item = Object::new();
        Reflect::set(&new_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&new_item, &JsValue::from_str("name"), &JsValue::from_str("Test Item")).unwrap();

        let op = Operation {
            collection: "demo".to_string(),
            op_type: OpType::CREATE,
            data: new_item.clone().into(),
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from("1234"))
        };

        // Test successful creation
        let created = db.write(op).await.unwrap();
        assert_eq!(
            Reflect::get(&created, &JsValue::from_str("id")).unwrap(),
            JsValue::from_str("1234")
        );

        // Test document retrieval
        let found = db
            .find_document_by_id("demo", JsValue::from_str("1234"))
            .await
            .unwrap();
        assert_eq!(
            Reflect::get(&found, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Test Item")
        );

        // Clean up
        db.close().await.unwrap();
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_find() {
        let schemas_obj = Object::new();
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "age": { "type": "number" },
                "status": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();

        let  db = IndexDB::create("test_db_find", schemas_obj).await.unwrap();

        // Create test documents
        let items = vec![
            json_str_to_js_value(r#"{
                "id": "1", "name": "Alice", "age": 30, "status": "active"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "2", "name": "Bob", "age": 25, "status": "inactive"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "3", "name": "Charlie", "age": 35, "status": "active"
            }"#).unwrap(),
        ];

        for item in items {
            let create_op = Operation {
                collection: "demo".to_string(),
                op_type: OpType::CREATE,
                data: item,
                primary_key_field: Some("id".to_string()),
                primary_key: Some(JsValue::from("1234"))
            };
            db.write(create_op).await.unwrap();
        }

        // Test find with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active",
            "age": { "$gt": 30 }
        }"#).unwrap();
        let query_options = QueryOptions {
            limit: None,
            offset: None
        };
        let result = db.find_js("demo", query_value, query_options).await.unwrap();
        let result_array = Array::from(&result);

        assert_eq!(result_array.length(), 1);
        let first_doc = result_array.get(0);
        assert_eq!(
            Reflect::get(&first_doc, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Charlie")
        );

        // Clean up
        db.close().await.unwrap();
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_count() {
        let schemas_obj = Object::new();
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "status": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();

        let  db = IndexDB::create("test_db_count", schemas_obj).await.unwrap();

        // Create test documents
        let items = vec![
            json_str_to_js_value(r#"{
                "id": "1", "name": "Alice", "status": "active"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "2", "name": "Bob", "status": "inactive"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "3", "name": "Charlie", "status": "active"
            }"#).unwrap(),
        ];

        for item in items {
            let primary_key = Reflect::get(
                &item,
                &JsValue::from_str("id")
            ).unwrap();
            let create_op = Operation {
                collection: "demo".to_string(),
                op_type: OpType::CREATE,
                data: item,
                primary_key_field: Some("id".to_string()),
                primary_key: Some(primary_key)
            };
            db.write(create_op).await.unwrap();
        }

        // Test count with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active"
        }"#).unwrap();
        let query_options = QueryOptions {
            limit: None,
            offset: None
        };
        let result = &db.count_js("demo", query_value, query_options).await.unwrap();
        assert_eq!(result.as_f64().unwrap(), 2.0);

        // Clean up
        db.close().await.unwrap();
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_multiple_collections() {
        // Create schemas for two collections
        let schemas_obj = Object::new();

        // Schema for users collection
        let users_schema = json_str_to_js_value(r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" }
            }
        }"#).unwrap();

        // Schema for products collection
        let products_schema = json_str_to_js_value(r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "price": { "type": "number" }
            }
        }"#).unwrap();

        Reflect::set(&schemas_obj, &JsValue::from_str("users"), &users_schema).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("products"), &products_schema).unwrap();

        let db = IndexDB::create("test_db_multiple_collections", schemas_obj).await.unwrap();

        // Insert data only into users collection
        let user = json_str_to_js_value(r#"{
            "id": "1",
            "name": "John Doe",
            "email": "john@example.com"
        }"#).unwrap();

        let create_op = Operation {
            collection: "users".to_string(),
            op_type: OpType::CREATE,
            data: user,
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from("1"))
        };

        db.write(create_op).await.unwrap();

        // Query the empty products collection
        let empty_query = json_str_to_js_value("{}").unwrap();
        let query_options = QueryOptions {
            limit: None,
            offset: None
        };
        // Find all products (should be empty)
        let products_result = db.find_js("products", empty_query.clone(), query_options).await.unwrap();
        let products_array = Array::from(&products_result);
        assert_eq!(products_array.length(), 0);

        // Count products (should be 0)
        let count_result = db.count_js("products", empty_query, query_options).await.unwrap();
        assert_eq!(count_result.as_f64().unwrap(), 0.0);

        // Clean up
        db.close().await.unwrap();
    }
}

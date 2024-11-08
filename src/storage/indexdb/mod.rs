use js_sys::{Array, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen_futures::JsFuture;
use crate::query::Query;
use crate::schema::Schema;
use crate::storage::base::StorageBase;
use crate::storage::internals::base_storage::BaseStorage;
use crate::storage::internals::core::CoreStorage;
use crate::operation::{OpType, Operation};
use web_sys::{IdbDatabase, IdbOpenDbRequest, IdbRequest, console};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents an IndexDB storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
export class IndexDB<T extends SchemaType> extends BaseStorage<T> {
    /**
     * Frees the resources used by the IndexDB storage.
     */
    free(): void;

    static create<TS extends SchemaType>(
        name: string,
        schema_type: TS,
        migrations: MigrationPathsForSchema<TS>,
    ): Promise<IndexDB<TS>>;
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct IndexDB {
    core: CoreStorage,
    base: BaseStorage,
    db: IdbDatabase,
} 


impl StorageBase for IndexDB {
    async fn write(&mut self, op: &Operation) -> Result<JsValue, JsValue> {
        console::log_1(&JsValue::from_str("Starting write operation..."));
        let store_name = "documents";
        
        let transaction = match self.db.transaction_with_str_and_mode(
            store_name,
            web_sys::IdbTransactionMode::Readwrite,
        ) {
            Ok(t) => t,
            Err(e) => {
                console::error_1(&JsValue::from_str("Failed to create transaction"));
                return Err(e);
            }
        };

        let store = match transaction.object_store(store_name) {
            Ok(s) => s,
            Err(e) => {
                console::error_1(&JsValue::from_str("Failed to get object store"));
                return Err(e);
            }
        };

        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                console::log_1(&JsValue::from_str("Processing CREATE/UPDATE operation"));
                let document = op.data.clone();
                
                // Extract primary key
                let primary_key = self.base.schema.primary_key.clone();
                let pk_value = match Reflect::get(&document, &JsValue::from_str(&primary_key)) {
                    Ok(v) => v,
                    Err(e) => {
                        console::error_2(&JsValue::from_str("Failed to get primary key value:"), &e);
                        return Err(e);
                    }
                };

                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Document must contain a primary key"));
                }

                // Validate document against schema
                self.base.schema.validate_schema(document.clone())?;

                // Store the document
                let _ = store.put_with_key(&document, &pk_value)?;
                
                Ok(document)
            },
            OpType::DELETE => {
                let pk_value = op.data.clone();
                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Primary key value is required for delete operation"));
                }

                let _ = store.delete(&pk_value)?;
                Ok(JsValue::from_str("Document deleted"))
            },
            _ => Err(JsValue::from_str("Unsupported operation type")),
        }
    }

    async fn find(&self, query: Query) -> Result<JsValue, JsValue> {
        console::log_2(&JsValue::from_str("Starting find operation with query:"), &query.query);
        let store_name = "documents";
        let transaction = self.db.transaction_with_str(store_name)?;
        let store = transaction.object_store(store_name)?;
        
        let normalized_query = query.parse()?;
        let request = store.get_all()?;
        let normalized_query = normalized_query.clone();
        let promise = Promise::new(&mut |resolve, reject| {
            let value = normalized_query.clone();
            let core = self.core.clone();
            let onsucess = Closure::once(Box::new(move |event: web_sys::Event| {
                let request: IdbRequest = event.target().unwrap().dyn_into().unwrap();
                let result = request.result().unwrap();
                let documents = Array::from(&result);
                // Filter documents based on query
                let filtered = Array::new();
                for i in 0..documents.length() {
                    let doc = documents.get(i);
                    if let Ok(matches) = core.document_matches_query(&doc, &value) {
                        if matches {
                            filtered.push(&doc);
                        }
                    }
                }
                
                resolve.call1(&JsValue::undefined(), &filtered).unwrap();
            }));
            
            request.set_onsuccess(Some(onsucess.as_ref().unchecked_ref()));
            onsucess.forget();
        });

        JsFuture::from(promise).await
    }

    async fn find_document_by_id(&self, primary_key_value: JsValue) -> Result<JsValue, JsValue> {
        console::log_2(&JsValue::from_str("Finding document by ID:"), &primary_key_value);
        let store_name = "documents";
        let transaction = self.db.transaction_with_str(store_name)?;
        let store = transaction.object_store(store_name)?;
        
        let request = store.get(&primary_key_value)?;
        
        let promise = Promise::new(&mut |resolve, reject| {
            let onsucess = Closure::once(Box::new(move |event: web_sys::Event| {
                let request: IdbRequest = event.target().unwrap().dyn_into().unwrap();
                let result = request.result().unwrap();
                
                if result.is_undefined() {
                    reject.call1(&JsValue::undefined(), &JsValue::from_str("Document not found")).unwrap();
                } else {
                    resolve.call1(&JsValue::undefined(), &result).unwrap();
                }
            }));
            
            request.set_onsuccess(Some(onsucess.as_ref().unchecked_ref()));
            onsucess.forget();
        });

        JsFuture::from(promise).await
    }

    async fn count(&self, query: Query) -> Result<JsValue, JsValue> {
        let store_name = "documents";
        let transaction = self.db.transaction_with_str(store_name)?;
        let store = transaction.object_store(store_name)?;
        
        let normalized_query = query.parse()?;
        let request = store.get_all()?;
        let normalized_query = normalized_query.clone();        
        let promise = Promise::new(&mut |resolve, reject| {
            let value = normalized_query.clone();
            let core = self.core.clone();
            let onsucess = Closure::once(Box::new(move |event: web_sys::Event| {
                let request: IdbRequest = event.target().unwrap().dyn_into().unwrap();
                let result = request.result().unwrap();
                let documents = Array::from(&result);
                
                let mut count = 0;
                for i in 0..documents.length() {
                    let doc = documents.get(i);
                    if let Ok(matches) = core.document_matches_query(&doc, &value) {
                        if matches {
                            count += 1;
                        }
                    }
                }
                
                resolve.call1(&JsValue::undefined(), &JsValue::from_f64(count as f64)).unwrap();
            }));
            
            request.set_onsuccess(Some(onsucess.as_ref().unchecked_ref()));
            onsucess.forget();
        });

        JsFuture::from(promise).await
    }

    async fn close(&self) -> Result<JsValue, JsValue> {
        self.db.close();
        Ok(JsValue::from_str("IndexDB database closed"))
    }
}



#[wasm_bindgen]
impl IndexDB {
    #[wasm_bindgen]
    pub async fn create(name: &str, schema_type: JsValue, migrations: JsValue) -> Result<IndexDB, JsValue> {
        // Ensure wasm_bindgen is initialized
        console::log_1(&JsValue::from_str("[IndexDB] Starting database creation..."));
        
        // Validate inputs
        if name.is_empty() {
            console::error_1(&JsValue::from_str("[IndexDB] Database name cannot be empty"));
            return Err(JsValue::from_str("Database name cannot be empty"));
        }

        if schema_type.is_undefined() || schema_type.is_null() {
            console::error_1(&JsValue::from_str("[IndexDB] Schema type is required"));
            return Err(JsValue::from_str("Schema type is required"));
        }

        console::log_2(&JsValue::from_str("[IndexDB] Creating database with name:"), &JsValue::from_str(name));
        console::log_2(&JsValue::from_str("[IndexDB] Schema:"), &schema_type);
        
        // Create BaseStorage with error handling
        let base = match BaseStorage::new(name.to_string(), schema_type, migrations) {
            Ok(b) => {
                console::log_1(&JsValue::from_str("[IndexDB] BaseStorage created successfully"));
                b
            },
            Err(e) => {
                console::error_2(&JsValue::from_str("[IndexDB] Failed to create BaseStorage:"), &e);
                return Err(e);
            }
        };
        
        // Get window object with explicit error handling
        let window = match web_sys::window() {
            Some(win) => win,
            None => {
                console::error_1(&JsValue::from_str("[IndexDB] Failed to get window object"));
                return Err(JsValue::from_str("Failed to get window object"));
            }
        };

        // Get IndexedDB with explicit error handling
        let idb = match window.indexed_db() {
            Ok(Some(idb)) => {
                console::log_1(&JsValue::from_str("[IndexDB] Successfully got IndexedDB"));
                idb
            },
            Ok(None) => {
                console::error_1(&JsValue::from_str("[IndexDB] IndexedDB not available"));
                return Err(JsValue::from_str("IndexedDB not available"));
            }
            Err(e) => {
                console::error_2(&JsValue::from_str("[IndexDB] Failed to get IndexedDB:"), &e);
                return Err(e);
            }
        };
        
        // Open database with explicit error handling
        let db_request = match idb.open_with_u32(name, 1) {
            Ok(req) => {
                console::log_1(&JsValue::from_str("[IndexDB] Database open request created"));
                req
            },
            Err(e) => {
                console::error_2(&JsValue::from_str("[IndexDB] Failed to open database:"), &e);
                return Err(e);
            }
        };

        // Set up error handler
        let onerror = Closure::once(Box::new(move |e: web_sys::Event| {
            console::error_2(&JsValue::from_str("[IndexDB] Database open error:"), &e);
        }));
        db_request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        
        // Set up upgrade handler
        let onupgradeneeded = Closure::once(Box::new(move |event: web_sys::Event| -> Result<(), JsValue> {
            console::log_1(&JsValue::from_str("[IndexDB] Running database upgrade..."));
            
            let db_request: IdbOpenDbRequest = match event.target() {
                Some(target) => match target.dyn_into::<IdbOpenDbRequest>() {
                    Ok(req) => req,
                    Err(e) => {
                        console::error_1(&JsValue::from_str("[IndexDB] Failed to get request from event"));
                        return Err(e.into());
                    }
                },
                None => {
                    console::error_1(&JsValue::from_str("[IndexDB] No target in upgrade event"));
                    return Err(JsValue::from_str("No target in upgrade event"));
                }
            };

            let db: IdbDatabase = match db_request.result() {
                Ok(res) => match res.dyn_into() {
                    Ok(db) => db,
                    Err(e) => {
                        console::error_1(&JsValue::from_str("[IndexDB] Failed to convert result to database"));
                        return Err(e);
                    }
                },
                Err(e) => {
                    console::error_1(&JsValue::from_str("[IndexDB] Failed to get database from request"));
                    return Err(e);
                }
            };
            
            if !db.object_store_names().contains(&"documents") {
                console::log_1(&JsValue::from_str("[IndexDB] Creating 'documents' object store"));
                match db.create_object_store("documents") {
                    Ok(_) => console::log_1(&JsValue::from_str("[IndexDB] Object store created successfully")),
                    Err(e) => {
                        console::error_1(&JsValue::from_str("[IndexDB] Failed to create object store"));
                        return Err(e);
                    }
                }
            }
            Ok(())
        }));
        
        db_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        
        // Create promise for database opening
        let promise = Promise::new(&mut |resolve, reject| {
            let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
                console::log_1(&JsValue::from_str("[IndexDB] Database opened successfully"));
                let request: IdbOpenDbRequest = event.target().unwrap().dyn_into().unwrap();
                let db: IdbDatabase = request.result().unwrap().dyn_into().unwrap();
                resolve.call1(&JsValue::undefined(), &db).unwrap();
            }));
            
            db_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
            onsuccess.forget();
        });

        // Wait for database to open
        console::log_1(&JsValue::from_str("[IndexDB] Waiting for database to open..."));
        let db = match JsFuture::from(promise).await {
            Ok(db) => db,
            Err(e) => {
                console::error_2(&JsValue::from_str("[IndexDB] Failed to open database:"), &e);
                return Err(e);
            }
        };

        Ok(IndexDB {
            base,
            core: CoreStorage {},
            db: db.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> Schema {
        self.base.schema.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.base.name.clone()
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&mut self, op: &Operation) -> Result<JsValue, JsValue> {
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "find")]
    pub async fn find_js(&self, query: JsValue) -> Result<JsValue, JsValue> {
        self.find(Query::new(query, self.schema())?).await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(&self, primary_key: JsValue) -> Result<JsValue, JsValue> {
        self.find_document_by_id(primary_key).await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self, query: JsValue) -> Result<JsValue, JsValue> {
        self.count(Query::new(query, self.schema())?).await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&self) -> Result<JsValue, JsValue> {
        self.close().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::Object;
    use wasm_bindgen_test::*;
    use serde_json::Value;
    use wasm_bindgen::JsCast;

    // Configure tests to run in browser
    wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to ensure window is available
    fn setup_window() -> web_sys::Window {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window().expect("no global `window` exists")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            panic!("Tests must be run in a browser environment");
        }
    }

    // Helper function to clean up IndexDB between tests
    async fn cleanup_db(name: &str) -> Result<(), JsValue> {
        let window = setup_window();
        let idb = window.indexed_db()?.expect("IndexDB not available");
        let delete_req = idb.delete_database(name)?;
        
        let promise = Promise::new(&mut |resolve, reject| {
            let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
                resolve.call0(&JsValue::undefined()).unwrap();
            }));
            
            let onerror = Closure::once(Box::new(move |e: web_sys::Event| {
                reject.call1(&JsValue::undefined(), &e).unwrap();
            }));
            
            delete_req.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
            delete_req.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            onsuccess.forget();
            onerror.forget();
        });

        JsFuture::from(promise).await?;
        Ok(())
    }

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
        let schema_str = "{ \"version\": 1, \"primaryKey\": \"id\", \"type\": \"object\", \"properties\": { \"id\": { \"type\": \"string\", \"maxLength\": 60 } } }";
        let schema_name = "test_empty_db";
        
        // Clean up any existing database
        let _ = cleanup_db(schema_name).await;
        
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();
        let indexdb = IndexDB::create(schema_name, schema, migrations).await;
        assert!(indexdb.is_ok());
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_write() {
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" }
            }
        }"#;
        let schema_name = "test_write_db";
        
        // Clean up any existing database
        let _ = cleanup_db(schema_name).await;
        
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();

        let mut indexdb = IndexDB::create(schema_name, schema, migrations).await.unwrap();

        // Create a new item
        let new_item = Object::new();
        Reflect::set(&new_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&new_item, &JsValue::from_str("name"), &JsValue::from_str("Test Item")).unwrap();

        let op = Operation {
            collection: schema_name.to_string(),
            op_type: OpType::CREATE,
            data: new_item.into(),
            indexes: vec![],
        };

        let created = indexdb.write(&op).await.unwrap();
        assert_eq!(
            Reflect::get(&created, &JsValue::from_str("id")).unwrap(),
            JsValue::from_str("1234")
        );

        // Try to retrieve the document
        let found = indexdb
            .find_document_by_id(JsValue::from_str("1234"))
            .await
            .unwrap();

        assert_eq!(
            Reflect::get(&found, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Test Item")
        );
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_update_operation() {
        let schema_name = "test_update_db";
        let _ = cleanup_db(schema_name).await;
        
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();

        let mut indexdb = IndexDB::create(schema_name, schema, migrations).await.unwrap();

        // Create initial item
        let new_item = Object::new();
        Reflect::set(&new_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&new_item, &JsValue::from_str("name"), &JsValue::from_str("Test Item")).unwrap();

        let create_op = Operation {
            collection: schema_name.to_string(),
            op_type: OpType::CREATE,
            data: new_item.into(),
            indexes: vec![],
        };

        indexdb.write(&create_op).await.unwrap();

        // Update the item
        let updated_item = Object::new();
        Reflect::set(&updated_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&updated_item, &JsValue::from_str("name"), &JsValue::from_str("Updated Item")).unwrap();

        let update_op = Operation {
            collection: schema_name.to_string(),
            op_type: OpType::UPDATE,
            data: updated_item.into(),
            indexes: vec![],
        };

        let updated = indexdb.write(&update_op).await.unwrap();
        assert_eq!(
            Reflect::get(&updated, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Updated Item")
        );

        // Verify update
        let found = indexdb
            .find_document_by_id(JsValue::from_str("1234"))
            .await
            .unwrap();
        assert_eq!(
            Reflect::get(&found, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Updated Item")
        );
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_delete_operation() {
        let schema_name = "test_delete_db";
        let _ = cleanup_db(schema_name).await;
        
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();

        let mut indexdb = IndexDB::create(schema_name, schema, migrations).await.unwrap();

        // Create initial item
        let new_item = Object::new();
        Reflect::set(&new_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&new_item, &JsValue::from_str("name"), &JsValue::from_str("Test Item")).unwrap();

        let create_op = Operation {
            collection: schema_name.to_string(),
            op_type: OpType::CREATE,
            data: new_item.into(),
            indexes: vec![],
        };

        indexdb.write(&create_op).await.unwrap();

        // Delete the item
        let delete_op = Operation {
            collection: schema_name.to_string(),
            op_type: OpType::DELETE,
            data: JsValue::from_str("1234"),
            indexes: vec![],
        };

        let delete_result = indexdb.write(&delete_op).await.unwrap();
        assert_eq!(delete_result, JsValue::from_str("Document deleted"));

        // Verify deletion
        let find_result = indexdb.find_document_by_id(JsValue::from_str("1234")).await;
        assert!(find_result.is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_find() {
        let schema_name = "test_find_db";
        let _ = cleanup_db(schema_name).await;
        
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" },
                "age": { "type": "number" },
                "status": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();
        let mut indexdb = IndexDB::create(schema_name, schema, migrations).await.unwrap();

        // Create test items
        let items = vec![
            json_str_to_js_value(r#"{
                "id": "1",
                "name": "Alice",
                "age": 30,
                "status": "active"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "2",
                "name": "Bob",
                "age": 25,
                "status": "inactive"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "3",
                "name": "Charlie",
                "age": 35,
                "status": "active"
            }"#).unwrap(),
        ];

        for item in items {
            let create_op = Operation {
                collection: schema_name.to_string(),
                op_type: OpType::CREATE,
                data: item,
                indexes: vec![],
            };
            indexdb.write(&create_op).await.unwrap();
        }

        // Test query
        let query_value = json_str_to_js_value(r#"{
            "status": "active",
            "age": { "$gt": 30 }
        }"#).unwrap();
        let query = Query::new(query_value, indexdb.schema()).unwrap();
        let result = indexdb.find(query).await.unwrap();

        let result_array = Array::from(&result);
        assert_eq!(result_array.length(), 1);
        let first_doc = result_array.get(0);
        assert_eq!(
            Reflect::get(&first_doc, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Charlie")
        );
    }

    #[wasm_bindgen_test(async)]
    async fn test_indexdb_storage_count() {
        let schema_name = "test_count_db";
        let _ = cleanup_db(schema_name).await;
        
        let schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string", "maxLength": 60 },
                "name": { "type": "string" },
                "age": { "type": "number" },
                "status": { "type": "string" }
            }
        }"#;
        let schema = json_str_to_js_value(schema_str).unwrap();
        let migrations = json_str_to_js_value("{}").unwrap();
        let mut indexdb = IndexDB::create(schema_name, schema, migrations).await.unwrap();

        // Create test items
        let items = vec![
            json_str_to_js_value(r#"{
                "id": "1",
                "name": "Alice",
                "age": 30,
                "status": "active"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "2",
                "name": "Bob",
                "age": 25,
                "status": "inactive"
            }"#).unwrap(),
            json_str_to_js_value(r#"{
                "id": "3",
                "name": "Charlie",
                "age": 35,
                "status": "active"
            }"#).unwrap(),
        ];

        for item in items {
            let create_op = Operation {
                collection: schema_name.to_string(),
                op_type: OpType::CREATE,
                data: item,
                indexes: vec![],
            };
            indexdb.write(&create_op).await.unwrap();
        }

        // Test count query
        let query_value = json_str_to_js_value(r#"{
            "status": "active"
        }"#).unwrap();
        let query = Query::new(query_value, indexdb.schema()).unwrap();
        let result = indexdb.count(query).await.unwrap();

        assert_eq!(result.as_f64().unwrap(), 2.0);
    }
}
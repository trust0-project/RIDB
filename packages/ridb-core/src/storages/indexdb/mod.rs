use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use pool::POOL;
use utils::{create_database, cursor_fetch_and_filter, get_indexed_fields_in_query, get_key_range, get_pks_from_index, idb_request_result};
use crate::utils::Logger;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
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
        // Clean pool occasionally to prevent stale connections
        POOL.clean_connections();
        
        let store_name = &op.collection;
        
        // Get schema information before acquiring store lock
        let primary_key_field;
        let primary_key_value;
        
        {
            let schemas = self.base.schemas.read();
            let schema = schemas.get(op.collection.as_str())
                .ok_or_else(|| RIDBError::from("Collection not found"))?;
            
            // Extract primary key field name
            primary_key_field = schema.primary_key.clone();
        }
        
        // Extract necessary information from the operation
        let document = op.data.clone();
        
        // Handle based on operation type
        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                // Extract primary key value before transaction
                primary_key_value = Reflect::get(&document, &JsValue::from_str(&primary_key_field))?;
                
                if primary_key_value.is_undefined() || primary_key_value.is_null() {
                    return Err(RIDBError::from("Document must contain a primary key"));
                }
                
                // Get the store - this creates a transaction
                let store = self.get_store(store_name)?;
                
                // Store the document and wait for completion
                let request = store.put_with_key(&document, &primary_key_value)?;
                
                // Wait for the request to complete
                idb_request_result(request).await?;
                
                // Return the document
                Ok(document.clone())
            },
            OpType::DELETE => {
                // Get the primary key value to delete
                primary_key_value = op.data.clone();
                
                if primary_key_value.is_undefined() || primary_key_value.is_null() {
                    return Err(RIDBError::from("Primary key value is required for delete operation"));
                }
                
                // Get the store - this creates a transaction
                let store = self.get_store(store_name)?;
                
                // Delete the document using the primary key
                let request = store.delete(&primary_key_value)?;
                
                // Wait for the request to complete
                idb_request_result(request).await?;
                
                // Return success message
                Ok(JsValue::from_str("Document deleted"))
            },
            _ => Err(RIDBError::from("Unsupported operation type")),
        }
    }

    async fn find(&self, collection_name: &str, query: Query, options: QueryOptions) -> Result<JsValue, RIDBError> {
        // Clean pool occasionally to prevent stale connections
        POOL.clean_connections();
        
        Logger::debug(
            "IndexDB-Find",
            &JsValue::from(format!("Find method {}", collection_name)),
        );

        let filtered_docs = self
            .collect_documents_for_query(collection_name, query, options)
            .await?;
        Ok(filtered_docs.into())
    }

    async fn find_document_by_id(&self, collection_name: &str, primary_key_value: JsValue) -> Result<JsValue, RIDBError> {
        // Clean pool occasionally to prevent stale connections
        POOL.clean_connections();
        
        let store_name = collection_name;
        if primary_key_value.is_undefined() || primary_key_value.is_null() {
            return Err(RIDBError::from("Primary key value is required"));
        }

        let store = self.get_store(store_name)?;

        Logger::debug("IndexDB-Find-By-Id", &JsValue::from(&format!("Finding document with primary key: {:?}", primary_key_value)));

        let request = store.get(&primary_key_value)?;

        let result = idb_request_result(request).await?;

        if result.is_undefined() || result.is_null() {
            Logger::debug("IndexDB-Find-By-Id",&JsValue::from("Document not found"));
            Ok(JsValue::null())
        } else {
            Logger::debug("IndexDB-Find-By-Id",&JsValue::from("Document found"));
            Ok(result)
        }
    }

    async fn count(&self, collection_name: &str, query: Query, options: QueryOptions) -> Result<JsValue, RIDBError> {
        // Clean pool occasionally to prevent stale connections
        POOL.clean_connections();
        
        Logger::debug(
            "IndexDB-Count",
            &JsValue::from(format!("Count method {}", collection_name)),
        );
        let filtered_docs = self
            .collect_documents_for_query(collection_name, query, options)
            .await?;
        Ok(JsValue::from_f64(filtered_docs.length() as f64))
    }
    async fn close(&self) -> Result<JsValue, RIDBError> {
        Logger::debug("IndexDB-Close", &JsValue::from_str("Starting close operation"));
        
        // Clean all connections before attempting to close this one
        POOL.clean_connections();
        Logger::debug("IndexDB-Close", &JsValue::from_str("Pool connections cleaned"));
        
        // First, extract the name so we can remove from pool later
        let db_name = self.base.name.clone();
        Logger::debug("IndexDB-Close", &JsValue::from_str(&format!("Closing database: {}", db_name)));
        
        // Remove the connection from the pool first
        Logger::debug("IndexDB-Close", &JsValue::from_str(&format!("Removing database connection from pool: {}", db_name)));
        POOL.remove_connection(&db_name);

        // Now close the database - keep lock scope as small as possible
        {
            Logger::debug("IndexDB-Close", &JsValue::from_str("Acquiring lock to close database"));
            let db = self.db.lock();
            Logger::debug("IndexDB-Close", &JsValue::from_str("Closing database connection"));
            db.close();
        }
        
        // Clear any stored handlers
        {
            Logger::debug("IndexDB-Close", &JsValue::from_str("Clearing stored handlers"));
            let mut error_handler = self._error_handler.lock();
            *error_handler = None;
            
            let mut success_handler = self._success_handler.lock();
            *success_handler = None;
        }
        
        Logger::debug("IndexDB-Close", &JsValue::from_str("Database close operation completed successfully"));
        Ok(JsValue::from_str("IndexDB database closed"))
    }

    async fn start(&self) -> Result<JsValue, RIDBError> {
        // Clean all connections before attempting to start this one
        POOL.clean_connections();
        Logger::debug("IndexDB-Start", &JsValue::from_str("Starting database"));
        
        // Save the database name before testing the connection
        let db_name = self.base.name.clone();
        
        // Test if database is closed by attempting a simple transaction
        let db_is_closed = {
            // Keep lock scope as small as possible
            let db_guard = self.db.lock();
            let store_names = db_guard.object_store_names();
            
            if store_names.length() == 0 {
                Logger::debug("IndexDB-Start", &JsValue::from_str("Database appears closed (no stores)"));
                true // No stores, likely closed or connection issue
            } else {
                // Don't unwrap, as this could cause panic if store_names is empty
                if let Some(test_store) = store_names.get(0) {
                    let is_err = db_guard.transaction_with_str(&test_store).is_err();
                    Logger::debug("IndexDB-Start", &JsValue::from_str(&format!("Test transaction failed: {}", is_err)));
                    is_err
                } else {
                    Logger::debug("IndexDB-Start", &JsValue::from_str("No store names available"));
                    true // Consider it closed if we can't get store names
                }
            }
        };
        
        if db_is_closed {
            // Database is closed, we need to reopen it
            Logger::debug("IndexDB-Start", &JsValue::from_str("Reopening closed database connection"));
            
            // Clone the schemas for create_database
            let schemas_clone = self.base.schemas.read().clone();
            
            // Create a new database connection
            Logger::debug("IndexDB-Start", &JsValue::from_str(&format!("Creating new database: {}", db_name)));
            let new_db = match create_database(&db_name, schemas_clone).await {
                Ok(db) => {
                    Logger::debug("IndexDB-Start", &JsValue::from_str("Database created successfully"));
                    db
                },
                Err(e) => {
                    // If we can't create a new database, make sure to clean up any dangling references
                    Logger::debug("IndexDB-Start", &JsValue::from_str(&format!("Failed to create database: {:?}", e)));
                    POOL.remove_connection(&db_name);
                    return Err(e);
                }
            };
            
            // Create a strong reference to store in the IndexDB struct
            let new_db_strong = Arc::clone(&new_db);
            
            // Update the pool with new connection - we use a weak reference in the pool
            Logger::debug("IndexDB-Start", &JsValue::from_str("Storing connection in pool"));
            POOL.store_connection(db_name, Arc::downgrade(&new_db));
            
            // Update our internal database reference
            {
                Logger::debug("IndexDB-Start", &JsValue::from_str("Updating internal database reference"));
                let mut db_guard = self.db.lock();
                *db_guard = (*new_db_strong).clone();
            }
            
            // Reset the handlers if needed
            {
                Logger::debug("IndexDB-Start", &JsValue::from_str("Resetting event handlers"));
                let mut error_handler = self._error_handler.lock();
                *error_handler = None;
                
                let mut success_handler = self._success_handler.lock();
                *success_handler = None;
            }
            
            Logger::debug("IndexDB-Start", &JsValue::from_str("Database started successfully"));
            Ok(JsValue::from_str("IndexDB database started"))
        } else {
            Logger::debug("IndexDB-Start", &JsValue::from_str("Database already started"));
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
        Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str(&format!("Collecting documents for query in {}", collection_name)));
        
        // Acquire references to the object store and schema
        let store = self.get_store_readonly(collection_name)?;
        let store = self.get_store_readonly(collection_name)?;
        let store = self.get_store_readonly(collection_name)?;
        let schemas = self.base.schemas.read();
        let schema = schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?
            .clone();

        let normalized_query_js = query.get_query()?;
        let query_obj = Object::from(normalized_query_js.clone());
        let keys = Object::keys(&query_obj);

        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(u32::MAX);
        let value_query = Query::new(query.clone().parse()?, query.schema.clone())?;

        if keys.length() == 1 && keys.get(0).as_string() == Some("$or".to_string()) {
            Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str("Handling $or query"));
            
            let or_clauses = Reflect::get(&query_obj, &keys.get(0))?.dyn_into::<Array>()?;
            let mut all_docs_map: HashMap<String, JsValue> = HashMap::new();
            let primary_key_field = schema.primary_key.clone();

            for i in 0..or_clauses.length() {
                let sub_query_js = or_clauses.get(i);
                let sub_query = Query::new(sub_query_js, schema.clone())?;

                let sub_query_options = QueryOptions { limit: None, offset: None };
                let docs_js = Box::pin(self.collect_documents_for_query(collection_name, sub_query, sub_query_options)).await?;
                let docs_array = Array::from(&docs_js);

                for j in 0..docs_array.length() {
                    let doc = docs_array.get(j);
                    let pk = Reflect::get(&doc, &JsValue::from_str(&primary_key_field))?;
                    let pk_str = self.core.get_primary_key_typed(pk)?;
                    all_docs_map.entry(pk_str).or_insert(doc);
                }
            }

            let docs: Vec<_> = all_docs_map.values().cloned().collect();
            let final_docs = docs.iter().skip(offset as usize).take(limit as usize).cloned().collect::<Vec<_>>();
            let results_array = Array::new();
            for doc in final_docs {
                results_array.push(&doc);
            }
            return Ok(results_array);
        }

        // Attempt to figure out if we can leverage a single index
        Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str("Checking for index optimization"));
        let indexed_fields = get_indexed_fields_in_query(&query, &schema)?;


        // Determine offset and limit
        Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str(&format!("Using offset: {}, limit: {}", offset, limit)));

        // Clone the query data for filtering
        let core = self.core.clone();

        // Prepare the final, filtered documents array
        // but efficiently fetch them using a cursor approach.
        let documents = if !indexed_fields.is_empty() && !query.has_or_operator() {
            Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str(&format!("Using indexes: {:?}", indexed_fields)));

            // Heuristic to pick the most selective index
            fn estimate_selectivity(value: &JsValue) -> u8 {
                if !value.is_object() || js_sys::Array::is_array(value) {
                    return 3; // direct equality value
                }
                let obj = Object::from(value.clone());
                let keys = Object::keys(&obj);
                if keys.length() == 1 {
                    let key = keys.get(0).as_string().unwrap_or_default();
                    match key.as_str() {
                        "$eq" => 3,
                        "$gt" | "$gte" | "$lt" | "$lte" => 2,
                        "$in" | "$nin" => 1,
                        _ => 0,
                    }
                } else if keys.length() == 2 {
                    3 // bounded range
                } else {
                    0
                }
            }

            let mut best_index: Option<(String, JsValue, u8)> = None;
            for field in indexed_fields {
                let index_value = query.get(&field)?;
                let score = estimate_selectivity(&index_value);
                let range_js = match get_key_range(&index_value)? {
                    Some(r) => r.into(),
                    None => JsValue::undefined(),
                };

                if best_index.as_ref().map(|(_, _, s)| *s).unwrap_or(0) < score {
                    best_index = Some((field.clone(), range_js, score));
                }
            }

            if let Some((best_field, range_js, score)) = best_index {
                // Only use the index cursor if we have a meaningful range or high selectivity
                if score >= 2 && !range_js.is_undefined() && !range_js.is_null() {
                    if let Ok(index) = store.index(&best_field) {
                        Logger::debug(
                            "IndexDB-CollectDocuments",
                            &JsValue::from_str(&format!("Using best index '{}' with range", best_field)),
                        );
                        let results = cursor_fetch_and_filter(
                            Some(&index),
                            None,
                            &range_js,
                            core,
                            value_query,
                            offset,
                            limit,
                        )
                        .await?;
                        Logger::debug(
                            "IndexDB-CollectDocuments",
                            &JsValue::from_str(&format!("Found {} results with index cursor", results.length())),
                        );
                        results
                    } else {
                        Logger::debug(
                            "IndexDB-CollectDocuments",
                            &JsValue::from_str("Failed to open index, falling back to store scan"),
                        );
                        cursor_fetch_and_filter(
                            None,
                            Some(&store.clone()),
                            &JsValue::undefined(),
                            core,
                            value_query,
                            offset,
                            limit,
                        )
                        .await?
                    }
                } else {
                    Logger::debug(
                        "IndexDB-CollectDocuments",
                        &JsValue::from_str("Index not selective enough or no range; falling back to store scan"),
                    );
                    cursor_fetch_and_filter(
                        None,
                        Some(&store.clone()),
                        &JsValue::undefined(),
                        core,
                        value_query,
                        offset,
                        limit,
                    )
                    .await?
                }
            } else {
                Logger::debug(
                    "IndexDB-CollectDocuments",
                    &JsValue::from_str("No suitable index found after evaluation; falling back to store scan"),
                );
                cursor_fetch_and_filter(
                    None,
                    Some(&store.clone()),
                    &JsValue::undefined(),
                    core,
                    value_query,
                    offset,
                    limit,
                )
                .await?
            }
        } else {
            Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str("No suitable index found, using full store scan"));
            // No single index is usable; fetch everything via cursor on the store
            let results = cursor_fetch_and_filter(
                None,
                Some(&store.clone()),
                &JsValue::undefined(),
                core,
                value_query,
                offset,
                limit,
            )
            .await?;
            Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str(&format!("Found {} results with full store scan", results.length())));
            results
        };

        Logger::debug("IndexDB-CollectDocuments", &JsValue::from_str("Document collection completed"));
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
    
    pub fn get_store(&self, store_name: &str) -> Result<IdbObjectStore, RIDBError> {
        Logger::debug("IndexDB-GetStore", &JsValue::from_str(&format!("Getting store: {}", store_name)));
        
        // First check if store exists
        let stores = self.get_stores();
        if !stores.contains(&store_name.to_string()) {
            Logger::debug("IndexDB-GetStore", &JsValue::from_str(&format!("Store '{}' does not exist", store_name)));
            return Err(RIDBError::from(&format!(
                "Store '{}' does not exist. Available stores: {:?}",
                store_name, stores
            )));
        }

        // Get a lock on the DB but release it quickly
        let transaction = {
            Logger::debug("IndexDB-GetStore", &JsValue::from_str("Acquiring database lock"));
            let db = self.db.lock(); // Minimize lock time
            Logger::debug("IndexDB-GetStore", &JsValue::from_str("Creating transaction"));
            db.transaction_with_str_and_mode(
                store_name,
                web_sys::IdbTransactionMode::Readwrite,
            )
        };

        // Process transaction result after releasing the lock
        let transaction = match transaction {
            Ok(t) => {
                Logger::debug("IndexDB-GetStore", &JsValue::from_str("Transaction created successfully"));
                t
            },
            Err(_e) => {
                Logger::debug("IndexDB-GetStore", &JsValue::from_str(&format!("Failed to create transaction for store '{}'", store_name)));
                return Err(RIDBError::from(&format!(
                    "Failed to create transaction for store '{}'",
                    store_name
                )));
            }
        };

        // Simplified transaction handling - no need for complex promise setup
        // The transaction will complete automatically when the object store operations are done
        Logger::debug("IndexDB-GetStore", &JsValue::from_str("Transaction created, proceeding with object store access"));

        // Get object store from transaction
        Logger::debug("IndexDB-GetStore", &JsValue::from_str(&format!("Getting object store: {}", store_name)));
        let store = transaction.object_store(store_name).map_err(|e| RIDBError::from(e))?;
        Logger::debug("IndexDB-GetStore", &JsValue::from_str("Object store obtained successfully"));
        
        Ok(store)
    }

    pub fn get_store_readonly(&self, store_name: &str) -> Result<IdbObjectStore, RIDBError> {
        Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str(&format!("Getting readonly store: {}", store_name)));

        let stores = self.get_stores();
        if !stores.contains(&store_name.to_string()) {
            Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str(&format!("Store '{}' does not exist", store_name)));
            return Err(RIDBError::from(&format!(
                "Store '{}' does not exist. Available stores: {:?}",
                store_name, stores
            )));
        }

        let transaction = {
            Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str("Acquiring database lock"));
            let db = self.db.lock();
            Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str("Creating readonly transaction"));
            db.transaction_with_str_and_mode(
                store_name,
                web_sys::IdbTransactionMode::Readonly,
            )
        };

        let transaction = match transaction {
            Ok(t) => t,
            Err(_e) => {
                Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str(&format!("Failed to create readonly transaction for store '{}'", store_name)));
                return Err(RIDBError::from(&format!(
                    "Failed to create readonly transaction for store '{}'",
                    store_name
                )));
            }
        };

        Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str(&format!("Getting object store: {}", store_name)));
        let store = transaction.object_store(store_name).map_err(|e| RIDBError::from(e))?;
        Logger::debug("IndexDB-GetStoreReadonly", &JsValue::from_str("Object store obtained successfully"));
        Ok(store)
    }

    #[wasm_bindgen]
    pub async fn create(name: &str, schemas_js: Object) -> Result<IndexDB, RIDBError> {
        Logger::debug("IndexDB-Create", &JsValue::from_str(&format!("Creating IndexDB: {}", name)));
        
        // Clean the pool before creating a new connection
        POOL.clean_connections();
        Logger::debug("IndexDB-Create", &JsValue::from_str("Pool connections cleaned"));
        
        // Create the base storage with the provided schemas
        Logger::debug("IndexDB-Create", &JsValue::from_str("Creating base storage"));
        let base = BaseStorage::new(
            name.to_string(),
            schemas_js.clone(),
            None
        )?;

        // Clone the schemas for create_database
        let schemas_clone = base.schemas.read().clone();
        
        // Try to get an existing connection from the pool
        Logger::debug("IndexDB-Create", &JsValue::from_str("Attempting to get existing connection from pool"));
        let db = match POOL.get_connection(name) {
            Some(db) => {
                Logger::debug("IndexDB-Create", &JsValue::from_str("Reusing existing database connection"));
                db
            },
            None => {
                // Create new connection if none exists
                Logger::debug("IndexDB-Create", &JsValue::from_str("Creating new database connection"));
                let db = create_database(name, schemas_clone).await?;
                
                // Store a weak reference in the pool to avoid circular references
                Logger::debug("IndexDB-Create", &JsValue::from_str("Storing connection in pool"));
                POOL.store_connection(name.to_string(), Arc::downgrade(&db));
                db
            }
        };
        
        // Wrap the database in an Arc<Mutex<>> for thread-safe access
        Logger::debug("IndexDB-Create", &JsValue::from_str("Creating thread-safe database wrapper"));
        let db_mutex = Arc::new(Mutex::new((*db).clone()));
        
        // Create the storage instance
        Logger::debug("IndexDB-Create", &JsValue::from_str("Finalizing IndexDB creation"));
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
        let schemas = self.base.schemas.read();
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
        let schemas = clone.base.schemas.read();
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

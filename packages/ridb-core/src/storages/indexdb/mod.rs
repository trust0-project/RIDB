use js_sys::{Array, Object, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen_futures::JsFuture;
use crate::logger::Logger;
use crate::query::Query;
use crate::storage::internals::base_storage::BaseStorage;
use crate::storage::internals::core::CoreStorage;
use crate::operation::{OpType, Operation};
use web_sys::{IdbDatabase, IdbFactory, IdbIndexParameters, IdbKeyRange, IdbObjectStore, IdbOpenDbRequest, IdbRequest};
use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Weak;
use lazy_static::lazy_static;
use crate::error::RIDBError;
use crate::query::options::QueryOptions;
use crate::schema::Schema;
use super::base::Storage;

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
pub struct IndexDB {
    core: CoreStorage,
    base: BaseStorage,
    db: IdbDatabase,
    _error_handler: Option<Closure<dyn FnMut(web_sys::Event)>>,
    _success_handler: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

impl Drop for IndexDB {
    fn drop(&mut self) {
        self._error_handler.take();
        self._success_handler.take();
        self.db.close();
    }
}

async fn idb_request_result(request: IdbRequest) -> Result<JsValue, JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let reject2 = reject.clone();
        let success_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            let request: IdbRequest = event.target()
                .unwrap()
                .dyn_into()
                .unwrap();

            match request.result() {
                Ok(result) => resolve.call1(&JsValue::undefined(), &result).unwrap(),
                Err(e) => reject.call1(&JsValue::undefined(), &e).unwrap(),
            }
        }));

        let error_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            let request: IdbRequest = event.target()
                .unwrap()
                .dyn_into()
                .unwrap();

            let error = request.error().unwrap();
            reject2.call1(&JsValue::undefined(), &error.unwrap()).unwrap();
        }));

        request.set_onsuccess(Some(success_callback.as_ref().unchecked_ref()));
        request.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        // The closures will automatically be dropped after the Promise resolves/rejects
        success_callback.forget();
        error_callback.forget();
    });

    JsFuture::from(promise).await
}

impl Storage for IndexDB {
    async fn write(&self, op: &Operation) -> Result<JsValue, RIDBError> {
        let store_name = &op.collection;
        let store = self.get_store(store_name)?;
        let schema = self.base.schemas.get(op.collection.as_str())
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

    async fn find(&self, collection_name: &str, query: &Query, options: &QueryOptions) -> Result<JsValue, RIDBError> {
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

    async fn count(&self, collection_name: &str, query: &Query, options: &QueryOptions) -> Result<JsValue, RIDBError> {
        let filtered_docs = self
            .collect_documents_for_query(collection_name, query, options)
            .await?;
        Ok(JsValue::from_f64(filtered_docs.length() as f64))
    }

    async fn close(&mut self) -> Result<JsValue, RIDBError> {
        // Wait for any pending transactions to complete
        let stores: Vec<String> = self.get_stores();

        // Create a read transaction for each store to ensure all operations are complete
        for store_name in stores {
            let transaction = self.db.transaction_with_str(&store_name)?;

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

        // Remove the connection from the pool
        POOL.remove_connection(&self.base.name);

        // Now safe to close the database
        self.db.close();
        Ok(JsValue::from_str("IndexDB database closed"))
    }

    async fn start(&mut self) -> Result<JsValue, RIDBError> {
        // Check if database is closed by attempting a simple transaction
        let test_store = self.db.object_store_names().get(0);
        if test_store.is_some() {
            let store_name = test_store.unwrap();
            if let Err(_) = self.db.transaction_with_str(&store_name) {
                let db = create_database(&self.base.name, &self.base.schemas).await?;
                // Update the pool with new connection
                POOL.store_connection(self.base.name.clone(), Arc::downgrade(&db));
                self.db = (*db).clone();
            }
            Ok(JsValue::from_str("IndexDB database started"))
        } else{
            Ok(JsValue::from_str("IndexDB database already started"))

        }

    }

}

/// Attempt to detect IndexedDB either in a Window or in a Worker scope
fn get_indexed_db() -> Result<IdbFactory, RIDBError> {
    // 1) If in a normal browser (Window) environment
    if let Ok(window) = js_sys::global().dyn_into::<web_sys::Window>() {
        if let Some(idb) = window.indexed_db()? {
            return Ok(idb);
        }
    }
    // 2) If in a Worker context
    else if let Ok(worker) = js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>() {
        if let Some(idb) = worker.indexed_db()? {
            return Ok(idb);
        }
    }

    Err(RIDBError::from("IndexedDB not available in this environment"))
}

async fn create_database(name: &str, schemas: &HashMap<String, Schema>) -> Result<Arc<IdbDatabase>, RIDBError> {
    let idb = get_indexed_db()?;

    let version = 1;
    let db_request = idb.open_with_u32(name, version)?;

    // Clone keys before entering the Promise

    let keys_vec: Vec<String> = schemas.keys()
        .map(|k| k.to_string())
        .collect();

    let db = JsFuture::from(Promise::new(&mut |resolve, reject| {
        let keys = keys_vec.clone();
        let schemas_clone = schemas.clone();
        let onupgradeneeded = Closure::once(Box::new(move |event: web_sys::Event| {
            let db: IdbDatabase = event.target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap()
                .result()
                .unwrap()
                .dyn_into()
                .unwrap();

            for collection_name in keys {
                let schema = schemas_clone.get(&collection_name).unwrap();
                if !db.object_store_names().contains(&collection_name) {
                    // Create object store
                    let object_store = db
                        .create_object_store(&collection_name)
                        .expect("Failed to create object store");

                    // If there are indexes, create them
                    if let Some(indexes) = &schema.indexes {
                        for index_name in indexes {
                            let mut index_params = IdbIndexParameters::new();
                            index_params.unique(false);
                            index_params.multi_entry(false);
                            Logger::debug(
                                "IndexDB",
                                &JsValue::from(
                                    format!(
                                        "Creating index in collection {} ::: {}",
                                        &collection_name,
                                        index_name
                                    )
                                )
                            );
                            object_store
                                .create_index_with_str_and_optional_parameters(
                                    index_name, // index name
                                    index_name, // key path
                                    &index_params,
                                )
                                .expect("Failed to create index");
                        }
                    }
                }
            }
        }));

        let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
            let db: IdbDatabase = event.target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap()
                .result()
                .unwrap()
                .dyn_into()
                .unwrap();
            resolve.call1(&JsValue::undefined(), &db).unwrap();
        }));

        let onerror = Closure::once(Box::new(move |e: web_sys::Event| {
            reject.call1(&JsValue::undefined(), &e).unwrap();
        }));

        db_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        db_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        db_request.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onupgradeneeded.forget();
        onsuccess.forget();
        onerror.forget();
    })).await?;

    Ok(Arc::new(db.dyn_into::<IdbDatabase>()?))
}

#[wasm_bindgen]
impl IndexDB {
    /// A helper function to collect and filter documents for a given query,
    /// respecting offsets and limits to avoid wasting time and resources.
    async fn collect_documents_for_query(
        &self,
        collection_name: &str,
        query: &Query,
        options: &QueryOptions
    ) -> Result<Array, RIDBError> {
        // Acquire references to the object store and schema
        let store = self.get_store(collection_name)?;
        let schema = self
            .base
            .schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?;

        // Attempt to figure out if we can leverage a single index
        let index_name_option = can_use_single_index_lookup(query, schema)?;

        // Determine offset and limit
        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(u32::MAX);

        // Clone the query data for filtering
        let core = self.core.clone();
        let normalized_query = query.parse()?;

        // Build a "value_query" for final in-memory filter
        let value_query = Query::new(normalized_query.clone(), query.schema.clone())?;

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
                        let key = key_array.get(i);
                        let partial_result = self
                            .cursor_fetch_and_filter(
                                Some(&index),
                                None,
                                &key,
                                &core,
                                &value_query,
                                offset,
                                limit,
                            )
                            .await?;
                        // Merge partial_result into merged_docs
                        for j in 0..partial_result.length() {
                            merged_docs.push(&partial_result.get(j));
                        }
                    }
                    merged_docs
                } else {
                    // Single key fetch from this index
                    self.cursor_fetch_and_filter(
                        Some(&index),
                        None,
                        &index_value,
                        &core,
                        &value_query,
                        offset,
                        limit,
                    )
                    .await?
                }
            } else {
                // If we couldn't get the index, do a cursor fetch for the entire store
                self.cursor_fetch_and_filter(
                    None,
                    Some(&store),
                    &JsValue::undefined(),
                    &core,
                    &value_query,
                    offset,
                    limit,
                )
                .await?
            }
        } else {
            // No single index is usable; fetch everything via cursor on the store
            self.cursor_fetch_and_filter(
                None,
                Some(&store),
                &JsValue::undefined(),
                &core,
                &value_query,
                offset,
                limit,
            )
            .await?
        };

        Ok(documents)
    }

    /// Fetch documents by opening an IndexedDB cursor (on an index or store),
    /// then apply inline filtering and offset/limit constraints.
    async fn cursor_fetch_and_filter(
        &self,
        index: Option<&web_sys::IdbIndex>,
        store: Option<&web_sys::IdbObjectStore>,
        key_value: &JsValue,
        core: &CoreStorage,
        value_query: &Query,
        offset: u32,
        limit: u32,
    ) -> Result<Array, RIDBError> {
        use std::cell::RefCell;
        use std::rc::Rc;

        let result_array = Rc::new(RefCell::new(Array::new()));
        let skipped_count = Rc::new(RefCell::new(0u32));
        let matched_count = Rc::new(RefCell::new(0u32));

        let promise = Promise::new(&mut |resolve, reject| {
            // Put `resolve` and `reject` into reference-counted pointers
            let resolve_rc = Rc::new(resolve);
            let reject_rc = Rc::new(reject);

            let result_array_cloned = result_array.clone();
            let skipped_count_cloned = skipped_count.clone();
            let matched_count_cloned = matched_count.clone();
            let core_cloned = core.clone();
            let value_query_cloned = value_query.clone();

            // Clone the Rc references so each closure can access them without moving.
            let resolve_for_success = Rc::clone(&resolve_rc);
            let reject_for_success = Rc::clone(&reject_rc);

            // On success callback: potentially invoked multiple times as we move the cursor
            let on_success = Closure::wrap(Box::new(move |evt: web_sys::Event| {
                let target: web_sys::IdbRequest = match evt.target().and_then(|t| t.dyn_into().ok()) {
                    Some(req) => req,
                    None => {
                        let _ = reject_for_success.call1(
                            &JsValue::NULL,
                            &JsValue::from_str("Failed to cast event target to IdbRequest."),
                        );
                        return;
                    }
                };

                let cursor_value = target.result();
                if cursor_value.is_err()
                    || cursor_value.as_ref().unwrap().is_null()
                    || cursor_value.as_ref().unwrap().is_undefined()
                {
                    // Cursor finished: resolve with the final array
                    let _ = resolve_for_success.call1(
                        &JsValue::NULL,
                        &result_array_cloned.borrow(),
                    );
                    return;
                }

                let cursor: web_sys::IdbCursorWithValue = match cursor_value.unwrap().dyn_into() {
                    Ok(c) => c,
                    Err(_) => {
                        let _ = reject_for_success.call1(
                            &JsValue::NULL,
                            &JsValue::from_str("Failed to cast cursor to IdbCursorWithValue."),
                        );
                        return;
                    }
                };

                let doc = match cursor.value() {
                    Ok(val) => val,
                    Err(err) => {
                        let _ = reject_for_success.call1(&JsValue::NULL, &err);
                        return;
                    }
                };

                // Filter in-memory based on the original query
                if core_cloned
                    .document_matches_query(&doc, &value_query_cloned)
                    .unwrap_or(false)
                {
                    let mut skip_ref = skipped_count_cloned.borrow_mut();
                    let mut match_ref = matched_count_cloned.borrow_mut();

                    if *skip_ref < offset {
                        *skip_ref += 1;
                    } else if *match_ref < limit {
                        result_array_cloned.borrow().push(&doc);
                        *match_ref += 1;
                    }
                    if *match_ref >= limit {
                        // Found enough docs: resolve immediately
                        let _ = resolve_for_success.call1(
                            &JsValue::NULL,
                            &result_array_cloned.borrow(),
                        );
                        return;
                    }
                }

                // Advance cursor
                if let Err(err) = cursor.continue_() {
                    let _ = reject_for_success.call1(&JsValue::NULL, &err);
                }
            }) as Box<dyn FnMut(_)>);

            // Clone again for on_error closure
            let reject_for_error = Rc::clone(&reject_rc);
            let on_error = Closure::wrap(Box::new(move |evt: web_sys::Event| {
                let _ = reject_for_error.call1(&JsValue::NULL, &evt);
            }) as Box<dyn FnMut(_)>);

            // Decide how to open the cursor
            let request_result = if let Some(idx) = index {
                if !key_value.is_null() && !key_value.is_undefined() {
                    match IdbKeyRange::only(key_value) {
                        Ok(range) => idx.open_cursor_with_range(&range),
                        Err(_) => idx.open_cursor(),
                    }
                } else {
                    idx.open_cursor()
                }
            } else if let Some(st) = store {
                if !key_value.is_null() && !key_value.is_undefined() {
                    match IdbKeyRange::only(key_value) {
                        Ok(range) => st.open_cursor_with_range(&range),
                        Err(_) => st.open_cursor(),
                    }
                } else {
                    st.open_cursor()
                }
            } else {
                Err(JsValue::from_str("No index or store provided to open cursor."))
            };

            // Attach success/error closures to the request
            match request_result {
                Ok(request) => {
                    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
                    request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

                    // Keep the closures alive for multiple invocations
                    on_success.forget();
                    on_error.forget();
                }
                Err(e) => {
                    let _ = reject_rc.call1(&JsValue::NULL, &e);
                }
            }
        });

        // Await the promise, then convert the result to an Array
        let js_result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(Array::from(&js_result))
    }

    pub fn get_stores(&self) -> Vec<String> {
        let store_names = self.db.object_store_names();
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
        let transaction = match self.db.transaction_with_str_and_mode(
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
        let base = BaseStorage::new(
            name.to_string(),
            schemas_js.clone(),
            None
        )?;
        let db = match POOL.get_connection(name) {
            Some(db) => db,
            None => {
                // Create new connection if none exists
                let db = create_database(name, &base.schemas).await?;
                POOL.store_connection(name.to_string(), Arc::downgrade(&db));
                db
            }
        };
        //base.add_index_schemas()?;
        Ok(IndexDB {
            base,
            core: CoreStorage {},
            db: (*db).clone(),
            _error_handler: None,
            _success_handler: None,
        })
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&self, op: &Operation) -> Result<JsValue, RIDBError> {
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "find")]
    pub async fn find_js(&self, collection_name: &str, query: JsValue, options: &QueryOptions) -> Result<JsValue, RIDBError> {
        let schema = self
            .base
            .schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let query = Query::new(query, schema.clone())?;
        self.find(collection_name, &query, options)
            .await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, RIDBError> {
        self.find_document_by_id(collection_name, primary_key).await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self, collection_name: &str, query: JsValue, options: &QueryOptions) -> Result<JsValue, RIDBError> {
        let schema = self
            .base
            .schemas
            .get(collection_name)
            .ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let query = Query::new(query, schema.clone())?;
        self.count(collection_name, &query, options)
            .await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&mut self) -> Result<JsValue, RIDBError> {
        self.close().await
    }

    #[wasm_bindgen(js_name = "start")]
    pub async fn start_js(&mut self) -> Result<JsValue, RIDBError> {
        self.start().await
    }
}
// Global connection pool
lazy_static! {
    static ref POOL: IndexDBPool = IndexDBPool::new();
}

// Add these trait implementations before the IndexDBPool struct
unsafe impl Send for IndexDBPool {}
unsafe impl Sync for IndexDBPool {}

pub struct IndexDBPool {
    connections: Arc<Mutex<HashMap<String, Arc<IdbDatabase>>>>,
}

impl IndexDBPool {
    fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    /// Retrieves a connection from the pool, recreating it if it's closed
    fn get_connection(&self, name: &str) -> Option<Arc<IdbDatabase>> {
        let mut connections = self.connections.lock();
        if let Some(db) = connections.get(name) {
            // Check if the database connection is still valid
            if db.is_closed() {
                // Remove the closed connection
                connections.remove(name);
                None
            } else {
                Some(db.clone())
            }
        } else {
            None
        }
    }

    /// Stores a new connection in the pool
    fn store_connection(&self, name: String, db: Weak<IdbDatabase>) {
        let mut connections = self.connections.lock();
        if let Some(arc_db) = db.upgrade() {
            connections.insert(name, arc_db);
        }
    }

    /// Removes a connection from the pool
    fn remove_connection(&self, name: &str) {
        let mut connections = self.connections.lock();
        connections.remove(name);
    }
}

// Add this extension trait
trait IdbDatabaseExt {
    fn is_closed(&self) -> bool;
}

impl IdbDatabaseExt for IdbDatabase {
    fn is_closed(&self) -> bool {
        // Attempt to start a dummy transaction to see if the database is closed
        match self.transaction_with_str("__non_existent_store__") {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}

fn can_use_single_index_lookup(
    query: &Query,
    schema: &Schema
) -> Result<Option<String>, RIDBError> {
    let fields = query.get_properties()?;
    let schema_indexes = &schema.indexes;
    if let Some(indexes) = schema_indexes {
        for index in indexes {
            if fields.contains(index) {
                return Ok(
                    Some(
                        index.clone()
                    )
                )
            }
        }
    }
    Ok(
        None
    )
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

        let mut db = IndexDB::create("test_db_create", schemas_obj).await.unwrap();

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
        let created = db.write(&op).await.unwrap();
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

        let mut db = IndexDB::create("test_db_find", schemas_obj).await.unwrap();

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
            db.write(&create_op).await.unwrap();
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
        let result = db.find_js("demo", query_value, &query_options).await.unwrap();
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

        let mut db = IndexDB::create("test_db_count", schemas_obj).await.unwrap();

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
            db.write(&create_op).await.unwrap();
        }

        // Test count with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active"
        }"#).unwrap();
        let query_options = QueryOptions {
            limit: None,
            offset: None
        };
        let result = db.count_js("demo", query_value, &query_options).await.unwrap();
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

        let mut db = IndexDB::create("test_db_multiple_collections", schemas_obj).await.unwrap();

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

        db.write(&create_op).await.unwrap();

        // Query the empty products collection
        let empty_query = json_str_to_js_value("{}").unwrap();
        let query_options = QueryOptions {
            limit: None,
            offset: None
        };
        // Find all products (should be empty)
        let products_result = db.find_js("products", empty_query.clone(), &query_options).await.unwrap();
        let products_array = Array::from(&products_result);
        assert_eq!(products_array.length(), 0);

        // Count products (should be 0)
        let count_result = db.count_js("products", empty_query, &query_options).await.unwrap();
        assert_eq!(count_result.as_f64().unwrap(), 0.0);

        // Clean up
        db.close().await.unwrap();
    }
}

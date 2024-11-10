use js_sys::{Array, Object, Promise, Reflect, JSON};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen_futures::JsFuture;
use crate::query::Query;
use crate::storage::internals::base_storage::BaseStorage;
use crate::storage::internals::core::CoreStorage;
use crate::operation::{OpType, Operation};
use web_sys::{IdbDatabase, IdbOpenDbRequest, IdbRequest, console};
use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Weak;
use lazy_static::lazy_static;

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

    static create<TS extends SchemaTypeRecord>(
        name: string,
        schema_type: any,
        migrations: any,
    ): Promise<IndexDB<TS>>;
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
    async fn write(&self, op: &Operation) -> Result<JsValue, JsValue> {
        let store_name = &op.collection;
        
        let transaction = match self.db.transaction_with_str_and_mode(
            store_name,
            web_sys::IdbTransactionMode::Readwrite,
        ) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let store = match transaction.object_store(store_name) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let schema = self.base.schemas.get(op.collection.as_str()).ok_or_else(|| JsValue::from_str("Collection not found"))?;

        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                let document = op.data.clone();
                
                // Extract primary key
                let primary_key = schema.primary_key.clone();
                let pk_value = match Reflect::get(&document, &JsValue::from_str(&primary_key)) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Document must contain a primary key"));
                }

                // Validate document against schema
                schema.validate_schema(document.clone())?;

                // Store the document and wait for completion
                let request = store.put_with_key(&document, &pk_value)?;
                idb_request_result(request).await?;

                Ok(document.clone())
            },
            OpType::DELETE => {
                let pk_value = op.data.clone();
                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Primary key value is required for delete operation"));
                }

                // Delete the document and wait for completion
                let request = store.delete(&pk_value)?;
                let promise = Promise::new(&mut |resolve, reject| {
                    let onsucess = Closure::once(Box::new(move |_event: web_sys::Event| {
                        resolve.call1(&JsValue::undefined(), &JsValue::from_str("Document deleted")).unwrap();
                    }));
                    
                    let onerror = Closure::once(Box::new(move |e: web_sys::Event| {
                        reject.call1(&JsValue::undefined(), &e).unwrap();
                    }));
                    
                    request.set_onsuccess(Some(onsucess.as_ref().unchecked_ref()));
                    request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                    onsucess.forget();
                    onerror.forget();
                });

                JsFuture::from(promise).await
            },
            _ => Err(JsValue::from_str("Unsupported operation type")),
        }
    }

    async fn find(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue> {
        let store_name = collection_name;
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
                // Filter documents based on query
                let filtered = Array::new();


                if !result.is_undefined() && !result.is_null() {
                    let documents = Array::from(&result);

                    for i in 0..documents.length() {
                        let doc = documents.get(i);
                        if let Ok(matches) = core.document_matches_query(&doc, &value) {
                            if matches {
                                filtered.push(&doc);
                            }
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

    async fn find_document_by_id(&self, collection_name: &str, primary_key_value: JsValue) -> Result<JsValue, JsValue> {
        let store_name = collection_name;
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

    async fn count(&self,collection_name: &str,   query: Query) -> Result<JsValue, JsValue> {
        let store_name = collection_name;
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

    fn schemas(&self) -> Result<JsValue, JsValue> {
        let schemas_js = Object::new();
        let schemas = self.base.schemas.clone();
        for (collection, schema) in schemas {
            let _ = Reflect::set(&schemas_js, &JsValue::from_str(&collection), &schema.into());
        }
        Ok(schemas_js.into())
    }

    async fn close(&self) -> Result<JsValue, JsValue> {
        self.db.close();
        Ok(JsValue::from_str("IndexDB database closed"))
    }

    async fn start(&mut self) -> Result<JsValue, JsValue> {
        // Check if database is closed by attempting a simple transaction
        let test_store = self.db.object_store_names().get(0);
        if test_store.is_some() {
            let store_name = test_store.unwrap();
            if let Err(_) = self.db.transaction_with_str(&store_name) {
                // Database is closed, need to reopen
                let schemas_js = Object::new();
                for (collection, schema) in self.base.schemas.iter() {
                    let _ = Reflect::set(&schemas_js, &JsValue::from_str(collection), &to_value(&schema)?);
                }
                let db = create_database(&self.base.name, &schemas_js).await?;
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

async fn create_database(name: &str, schemas_js: &Object) -> Result<Arc<IdbDatabase>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;
    let idb = window.indexed_db()?.ok_or_else(|| JsValue::from_str("IndexedDB not available"))?;
    
    let version = 1;
    let db_request = idb.open_with_u32(name, version)?;

    // Clone keys before entering the Promise
    let keys_array = Object::keys(schemas_js);
    let keys_vec: Vec<String> = (0..keys_array.length())
        .filter_map(|i| keys_array.get(i).as_string())
        .collect();
    
    let db = JsFuture::from(Promise::new(&mut |resolve, reject| {
        let keys = keys_vec.clone();
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
                if !db.object_store_names().contains(&collection_name) {
                    db.create_object_store(&collection_name)
                        .expect("Failed to create object store");
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
    #[wasm_bindgen]
    pub async fn create(name: &str, schemas_js: Object, migrations_js: Object) -> Result<IndexDB, JsValue> {
        let base = BaseStorage::new(
            name.to_string(),
            schemas_js.clone(),
            migrations_js
        )?;

        // Try to get existing connection from pool
        let db = match POOL.get_connection(name) {
            Some(db) => db,
            None => {
                // Create new connection if none exists
                let db = create_database(name, &schemas_js).await?;
                POOL.store_connection(name.to_string(), Arc::downgrade(&db));
                db
            }
        };

        Ok(IndexDB {
            base,
            core: CoreStorage {},
            db: (*db).clone(),
            _error_handler: None,
            _success_handler: None,
        })
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&self, op: &Operation) -> Result<JsValue, JsValue> {
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "find")]
    pub async fn find_js(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from_str("Collection not found"))?;
        self.find(collection_name, Query::new(query, schema.clone())?).await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue> {
        self.find_document_by_id(collection_name, primary_key).await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from_str("Collection not found"))?;
        self.count(collection_name, Query::new(query, schema.clone())?).await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&self) -> Result<JsValue, JsValue> {
        self.close().await
    }

    #[wasm_bindgen(js_name = "start")]
    pub async fn start_js(&mut self) -> Result<JsValue, JsValue> {
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

    fn get_connection(&self, name: &str) -> Option<Arc<IdbDatabase>> {
        let connections = self.connections.lock();
        if let Some(db) = connections.get(name) {
            Some(db.clone())
        } else {
            None
        }
    }

    fn store_connection(&self, name: String, db: Weak<IdbDatabase>) {
        let mut connections = self.connections.lock();
        if let Some(arc_db) = db.upgrade() {
            connections.insert(name, arc_db);
        }
    }
}


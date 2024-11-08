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
use web_sys::{IdbDatabase, IdbOpenDbRequest, IdbRequest};

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
        let store_name = "documents";
        let transaction = self.db.transaction_with_str_and_mode(
            store_name,
            web_sys::IdbTransactionMode::Readwrite,
        )?;
        let store = transaction.object_store(store_name)?;

        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                let document = op.data.clone();
                
                // Extract primary key
                let primary_key = self.base.schema.primary_key.clone();
                let pk_value = Reflect::get(&document, &JsValue::from_str(&primary_key))?;
                
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
        let base = BaseStorage::new(name.to_string(), schema_type, migrations)?;
        
        let window = web_sys::window().unwrap();
        let idb = window.indexed_db()?.unwrap();
        
        let db_request = idb.open_with_u32(name, 1)?;
        
        // Fixed closure with proper error handling
        let onupgradeneeded = Closure::once(Box::new(move |event: web_sys::Event| -> Result<(), JsValue> {
            let db_request: IdbOpenDbRequest = event.target().unwrap().dyn_into()?;
            let db: IdbDatabase = db_request.result()?.dyn_into()?;
            
            if !db.object_store_names().contains(&"documents") {
                db.create_object_store("documents")?;
            }
            Ok(())
        }));
        
        db_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        
        let promise = Promise::new(&mut |resolve, reject| {
            let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
                let request: IdbOpenDbRequest = event.target().unwrap().dyn_into().unwrap();
                let db: IdbDatabase = request.result().unwrap().dyn_into().unwrap();
                resolve.call1(&JsValue::undefined(), &db).unwrap();
            }));
            
            db_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
            onsuccess.forget();
        });

        let db = JsFuture::from(promise).await?.dyn_into::<IdbDatabase>()?;
        
        Ok(IndexDB {
            base,
            core: CoreStorage {},
            db,
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
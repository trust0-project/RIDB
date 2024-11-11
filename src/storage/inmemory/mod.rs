use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::operation::{OpType, Operation};
use crate::query::Query;
use crate::storage::internals::base_storage::BaseStorage;
use std::sync::RwLock;

use super::base::Storage;
use super::internals::core::CoreStorage;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents an in-memory storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
export class InMemory<T extends SchemaTypeRecord> extends BaseStorage<T> {
    /**
     * Frees the resources used by the in-memory storage.
     */
    free(): void;

    static create<TS extends SchemaTypeRecord>(
        name: string,
        schema_type: any,
        migrations: any,
    ): Promise<InMemory<TS>>;
}
"#;

#[derive(Debug)]
#[wasm_bindgen(skip_typescript)]
pub struct InMemory {
    core: CoreStorage,
    base: BaseStorage,
    by_index: RwLock<HashMap<String, HashMap<String, JsValue>>>,
}

impl Storage for InMemory {

    async fn write(&self, op: &Operation) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(op.collection.as_str()).ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let primary_key = schema.primary_key.clone();
        let index_name = format!("pk_{}", primary_key);

        let mut index_guard = self.by_index.write().map_err(|_| JsValue::from_str("Failed to acquire write lock"))?;
        let index = index_guard
            .entry(index_name.clone())
            .or_insert_with(HashMap::new);

        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                let document = op.data.clone();

                // Extract primary key
                let pk_value = Reflect::get(&document, &JsValue::from_str(&primary_key))
                    .map_err(|e| JsValue::from_str(&format!("Failed to get primary key: {:?}", e)))?;

                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Document must contain a primary key"));
                }

                let pk_str = if let Some(s) = pk_value.as_string() {
                    s
                } else if let Some(n) = pk_value.as_f64() {
                    n.to_string()
                } else {
                    return Err(JsValue::from_str("Primary key must be a string or number"));
                };

                match op.op_type {
                    OpType::CREATE => {
                        schema.validate_schema(document.clone())?;
                        
                        if index.contains_key(&pk_str) {
                            return Err(JsValue::from_str("Document with this primary key already exists"));
                        }
                        
                        index.insert(pk_str.clone(), document.clone());
                        Ok(document)
                    }
                    OpType::UPDATE => {
                        schema.validate_schema(document.clone())?;
                        
                        if !index.contains_key(&pk_str) {
                            return Err(JsValue::from_str("Document with this primary key does not exist"));
                        }
                        
                        index.insert(pk_str.clone(), document.clone());
                        Ok(document)
                    }
                    _ => Err(JsValue::from_str("Unsupported operation type for this data"))
                }
            }
            OpType::DELETE => {
                let pk_value = op.data.clone();

                if pk_value.is_undefined() || pk_value.is_null() {
                    return Err(JsValue::from_str("Primary key value is required for delete operation"));
                }

                let pk_str = if let Some(s) = pk_value.as_string() {
                    s
                } else if let Some(n) = pk_value.as_f64() {
                    n.to_string()
                } else {
                    return Err(JsValue::from_str("Primary key must be a string or number"));
                };

                if index.remove(&pk_str).is_some() {
                    Ok(JsValue::from_str("Document deleted"))
                } else {
                    Err(JsValue::from_str("Document with this primary key does not exist"))
                }
            }
            _ => Err(JsValue::from_str("Unsupported operation type"))
        }
    }

    async fn find(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from_str("Collection not found"))?;
        let normalized_query = query.parse()?;
        let results = Array::new();
        let primary_key = schema.primary_key.clone();
        let index_name = format!("pk_{}", primary_key);

        if let Some(index) = self.by_index.read().unwrap().get(&index_name) {
            for (_pk, doc) in index.iter() {
                let matches = self.core.document_matches_query(doc, &normalized_query)?;
                if matches {
                    results.push(doc);
                }
            }
        }

        Ok(results.into())
    }

    async fn find_document_by_id(
        &self,
        collection_name: &str,
        primary_key_value: JsValue,
    ) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from_str("Collection not found"))?;

        let primary_key = schema.primary_key.clone();
        let index_name = format!("pk_{}", primary_key);

        // Convert primary key value to string
        let pk_str = if let Some(s) = primary_key_value.as_string() {
            s
        } else if let Some(n) = primary_key_value.as_f64() {
            n.to_string()
        } else {
            return Err(JsValue::from_str("Invalid primary key value"));
        };

        // Retrieve the index
        if let Some(index) = self.by_index.read().unwrap().get(&index_name) {
            if let Some(doc) = index.get(&pk_str) {
                return Ok(doc.clone());
            }
        }

        Err(JsValue::from_str("Document not found"))
    }

    async fn count(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue> {
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from_str("Collection not found"))?;

        // Get the normalized query
        let normalized_query = query.parse()?;

        // Count matching documents
        let mut count = 0;

        // Get all documents from the primary key index
        let primary_key = schema.primary_key.clone();
        let index_name = format!("pk_{}", primary_key);
        if let Some(index) = self.by_index.read().unwrap().get(&index_name) {
            for (_pk, doc) in index.iter() {
                let matches = self.core.document_matches_query(doc, &normalized_query)?;
                if matches {
                    count += 1;
                }
            }
        }

        Ok(JsValue::from_f64(count as f64))
    }

    async fn close(&self) -> Result<JsValue, JsValue> {
        // Clear all data from the storage
        self.by_index.write()
            .map_err(|_| JsValue::from_str("Failed to acquire write lock"))?
            .clear();
        
        Ok(JsValue::from_str("In-memory database closed"))
    }

    async fn start(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_str("In-memory database started"))
    }
    
}


#[wasm_bindgen]
impl InMemory {
    
    #[wasm_bindgen]
    pub async fn create(name: &str, schemas_js: Object) -> Result<InMemory, JsValue> {
        let base_res = BaseStorage::new(
            name.to_string(),
            schemas_js,
        );
        match base_res {
            Ok(base) => Ok(
                InMemory {
                    base,
                    by_index: RwLock::new(HashMap::new()),
                    core: CoreStorage {}
                }
            ),
            Err(e) => Err(e)
        }
    }

    #[wasm_bindgen(getter)]
    pub fn by_index(&self) -> Result<JsValue, JsValue> {
        let guard = self.by_index.read().map_err(|_| JsValue::from_str("Failed to acquire read lock"))?;
        let outer_obj = Object::new();
        for (outer_key, inner_map) in &*guard {
            let inner_obj = Object::new();
            for (inner_key, value) in inner_map {
                Reflect::set(&inner_obj, &JsValue::from_str(inner_key), value)
                    .map_err(|_| {
                        JsValue::from_str("Failed to set inner object property")
                    })?;
            }
            Reflect::set(
                &outer_obj,
                &JsValue::from_str(outer_key),
                &JsValue::from(inner_obj),
            ).map_err(|_| {
                JsValue::from_str("Failed to set outer object property")
            })?;
        }
        Ok(JsValue::from(outer_obj))
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
    pub async fn find_document_by_id_js(
        &self,
        collection_name: &str,
        primary_key: JsValue,
    ) -> Result<JsValue, JsValue> {
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
    async fn test_empty_inmemory_storage() {
        let schemas_obj = Object::new();
        let schema_str = "{ \"version\": 1, \"primaryKey\": \"id\", \"type\": \"object\", \"properties\": { \"id\": { \"type\": \"string\", \"maxLength\": 60 } } }";
        let schema = json_str_to_js_value(schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();
        
        let inmem = InMemory::create("test_db", schemas_obj).await;
        assert!(inmem.is_ok());
    }

    #[wasm_bindgen_test(async)]
    async fn test_inmemory_storage_create_operation() {
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
        
        let inmem = InMemory::create("test_db", schemas_obj).await.unwrap();

        // Create a new item
        let new_item = Object::new();
        Reflect::set(&new_item, &JsValue::from_str("id"), &JsValue::from_str("1234")).unwrap();
        Reflect::set(&new_item, &JsValue::from_str("name"), &JsValue::from_str("Test Item")).unwrap();

        let op = Operation {
            collection: "demo".to_string(),
            op_type: OpType::CREATE,
            data: new_item.clone().into(),
            indexes: vec![],
        };

        // Test successful creation
        let created = inmem.write(&op).await.unwrap();
        assert_eq!(
            Reflect::get(&created, &JsValue::from_str("id")).unwrap(),
            JsValue::from_str("1234")
        );

        // Test document retrieval
        let found = inmem
            .find_document_by_id("demo", JsValue::from_str("1234"))
            .await
            .unwrap();
        assert_eq!(
            Reflect::get(&found, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Test Item")
        );

        // Test duplicate creation fails
        let duplicate_op = Operation {
            collection: "demo".to_string(),
            op_type: OpType::CREATE,
            data: new_item.into(),
            indexes: vec![],
        };

        let duplicate_result = inmem.write(&duplicate_op).await;
        assert!(duplicate_result.is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn test_inmemory_storage_find() {
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
        
        let  inmem = InMemory::create("test_db", schemas_obj).await.unwrap();

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
                indexes: vec![],
            };
            inmem.write(&create_op).await.unwrap();
        }

        // Test find with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active",
            "age": { "$gt": 30 }
        }"#).unwrap();
        
        let result = inmem.find_js("demo", query_value).await.unwrap();
        let result_array = Array::from(&result);
        
        assert_eq!(result_array.length(), 1);
        let first_doc = result_array.get(0);
        assert_eq!(
            Reflect::get(&first_doc, &JsValue::from_str("name")).unwrap(),
            JsValue::from_str("Charlie")
        );
    }

    #[wasm_bindgen_test(async)]
    async fn test_inmemory_storage_count() {
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
        
        let  inmem = InMemory::create("test_db", schemas_obj).await.unwrap();

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
            let create_op = Operation {
                collection: "demo".to_string(),
                op_type: OpType::CREATE,
                data: item,
                indexes: vec![],
            };
            inmem.write(&create_op).await.unwrap();
        }

        // Test count with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active"
        }"#).unwrap();
        
        let result = inmem.count_js("demo", query_value).await.unwrap();
        assert_eq!(result.as_f64().unwrap(), 2.0);
    }
}
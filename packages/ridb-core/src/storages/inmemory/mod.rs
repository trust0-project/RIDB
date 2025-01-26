use std::collections::{HashMap, HashSet};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::operation::{OpType, Operation};
use crate::query::Query;
use crate::storage::internals::base_storage::BaseStorage;
use crate::storage::internals::core::CoreStorage;
use std::sync::RwLock;
use crate::logger::Logger;
use super::base::Storage;

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

    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
    ): Promise<
        InMemory<
            SchemasCreate
        >
    >;
}
"#;

#[derive(Debug)]
#[wasm_bindgen(skip_typescript)]
pub struct InMemory {
    core: CoreStorage,
    base: BaseStorage,
    by_index: RwLock<HashMap<String, HashMap<String, JsValue>>>,
    started: bool,
}

impl Storage for InMemory {

    async fn write(&self, op: &Operation) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::write",
            &JsValue::from_str(&format!(
                "Write operation started. Collection: '{}', OpType: '{:?}', Primary Key Field: '{:?}'",
                op.collection, op.op_type, op.primary_key_field
            ))
        );

        let schema = self.base.schemas
            .get(op.collection.as_str())
            .ok_or_else(|| {
                let msg = "Collection not found".to_string();
                Logger::log(
                    "InMemory::write",
                    &JsValue::from_str(&format!("Error: {}", msg))
                );
                JsValue::from_str(&msg)
            })?;

        let primary_key = schema.primary_key.clone();
        let index_name = format!(
            "pk_{}_{}",
            op.collection,
            primary_key.clone()
        );

        let mut index_guard = self.by_index.write().map_err(|_| {
            let msg = "Failed to acquire write lock".to_string();
            Logger::log(
                "InMemory::write",
                &JsValue::from_str(&format!("Error: {}", msg))
            );
            JsValue::from_str(&msg)
        })?;
        
        match op.op_type {
            OpType::CREATE | OpType::UPDATE => {
                let document = op.data.clone();
                let pk_value = Reflect::get(&document, &JsValue::from_str(&primary_key))
                    .map_err(|e| {
                        let msg = format!("Failed to get primary key: {:?}", e);
                        Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                        JsValue::from_str(&msg)
                    })?;
                let pk_str = self.core.get_primary_key_typed(pk_value.clone())?;
                match op.op_type {
                    OpType::CREATE => {
                        Logger::log(
                            "InMemory::write",
                            &JsValue::from_str(&format!("CREATE operation for primary key: '{}'", pk_str))
                        );
                        {
                            let pk_map = index_guard
                                .entry(index_name.clone())
                                .or_insert_with(HashMap::new);
                            if pk_map.contains_key(&pk_str) {
                                let msg = "Document with this primary key already exists".to_string();
                                Logger::log(
                                    "InMemory::write",
                                    &JsValue::from_str(&format!("Error: {}", msg))
                                );
                                return Err(JsValue::from_str(&msg));
                            }
                            pk_map.insert(pk_str.clone(), document.clone());
                        }

                        let primary_index_name = op.primary_key_index()?;
                        let indexed_fields = schema.clone().indexes.unwrap_or(Vec::new());
                        for indexed_field in indexed_fields {
                            let collection_name = format!("idx_{}_{}", op.collection, indexed_field);
                            if collection_name == primary_index_name {
                                continue;
                            }
                            let indexed_value = Reflect::get(
                                &document,
                                &JsValue::from(indexed_field.clone())
                            )?;

                            if !indexed_value.is_null() && !indexed_value.is_undefined() {
                                let indexed_value_str = indexed_value
                                    .as_string()
                                    .unwrap_or_else(|| {
                                        indexed_value.as_f64().map(|num| num.to_string()).unwrap_or_default()
                                    });

                                let collection_index = index_guard
                                    .entry(collection_name.clone())
                                    .or_insert_with(HashMap::new);

                                let existing_entry = collection_index.get(&indexed_value_str);
                                let new_index_items = if let Some(existing_entry) = existing_entry {
                                    let items_val = Reflect::get(
                                        existing_entry,
                                        &JsValue::from_str("items")
                                    ).unwrap_or_else(|_| JsValue::from(js_sys::Array::new()));
                                    js_sys::Array::from(&items_val)
                                } else {
                                    js_sys::Array::new()
                                };

                                new_index_items.push(&pk_value.clone());

                                let item = js_sys::Object::new();
                                Reflect::set(
                                    &item,
                                    &JsValue::from("id"),
                                    &JsValue::from_str(&indexed_value_str)
                                )?;
                                Reflect::set(
                                    &item,
                                    &JsValue::from("items"),
                                    &JsValue::from(&new_index_items)
                                )?;

                                collection_index.insert(
                                    indexed_value_str,
                                    JsValue::from(&item)
                                );
                            }
                        }
                        Logger::log(
                            "InMemory::write",
                            &JsValue::from_str("CREATE operation completed successfully.")
                        );
                        Ok(document)
                    }
                    OpType::UPDATE => {
                        Logger::log(
                            "InMemory::write",
                            &JsValue::from_str(&format!("UPDATE operation for primary key: '{}'", pk_str))
                        );

                        let index = index_guard
                            .entry(index_name.clone())
                            .or_insert_with(HashMap::new);

                        if !index.contains_key(&pk_str) {
                            let msg = "Document with this primary key does not exist".to_string();
                            Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                            return Err(JsValue::from_str(&msg));
                        }
                        index.insert(pk_str.clone(), document.clone());
                        Logger::log("InMemory::write", &JsValue::from_str("UPDATE operation completed successfully."));
                        Ok(document)
                    }
                    _ => {
                        let msg = "Unsupported operation type for this data".to_string();
                        Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                        Err(JsValue::from_str(&msg))
                    }
                }
            }
            OpType::DELETE => {
                Logger::log(
                    "InMemory::write",
                    &JsValue::from_str("DELETE operation")
                );
                let pk_str = self.core.get_primary_key_typed(op.data.clone())?;
                if let Some(index) = index_guard.get_mut(&index_name) {
                    if index.remove(&pk_str).is_some() {
                        Logger::log("InMemory::write", &JsValue::from_str("DELETE operation completed successfully."));
                        Ok(JsValue::from_str("Document deleted"))
                    } else {
                        let msg = "Document with this primary key does not exist".to_string();
                        Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                        Err(JsValue::from_str(&msg))
                    }
                } else {
                    let msg = "Document with this primary key does not exist".to_string();
                    Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                    Err(JsValue::from_str(&msg))
                }
            }
            _ => {
                let msg = "Unsupported operation type".to_string();
                Logger::log("InMemory::write", &JsValue::from_str(&format!("Error: {}", msg)));
                Err(JsValue::from_str(&msg))
            }
        }
    }

    async fn find(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::find",
            &JsValue::from_str(&format!(
                "Find called for collection '{}', query: {:?}",
                collection_name,
                query
            ))
        );
        let documents = self.get_matching_documents(collection_name, &query).await?;
        let results_array = Array::new();

        for doc in documents {
            results_array.push(&doc);
        }

        Logger::log(
            "InMemory::find",
            &JsValue::from_str(&format!(
                "Find completed. Number of documents found: {}",
                results_array.length()
            ))
        );
        Ok(JsValue::from(results_array))
    }

    async fn find_document_by_id(
        &self,
        collection_name: &str,
        primary_key_value: JsValue,
    ) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::find_document_by_id",
            &JsValue::from_str(&format!(
                "Find document by ID called for collection '{}'.",
                collection_name
            ))
        );
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| {
            let msg = format!("Collection {} not found in findDocumentById", collection_name);
            Logger::log(
                "InMemory::find_document_by_id",
                &JsValue::from_str(&format!("Error: {}", msg))
            );
            JsValue::from(msg)
        })?;
        let primary_key = schema.primary_key.clone();

        let index_name = if collection_name.starts_with("idx_") {
            collection_name.to_string()
        } else {
            format!("pk_{}_{}", collection_name, primary_key)
        };

        if let Some(index) = self.by_index.read().unwrap().get(&index_name) {
            let pk_str = self.core.get_primary_key_typed(primary_key_value.clone())?;
            if let Some(doc) = index.get(&pk_str) {
                Logger::log(
                    "InMemory::find_document_by_id",
                    &JsValue::from_str("Document found.")
                );
                return Ok(doc.clone());
            }
        }
        Logger::log(
            "InMemory::find_document_by_id",
            &JsValue::from_str("Document not found.")
        );
        Ok(JsValue::undefined())
    }

    async fn count(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::count",
            &JsValue::from_str(&format!(
                "Count called for collection '{}', query: {:?}",
                collection_name,
                query
            ))
        );
        let documents = self.get_matching_documents(collection_name, &query).await?;
        Logger::log(
            "InMemory::count",
            &JsValue::from_str(&format!(
                "Count completed. Number of documents matching: {}",
                documents.len()
            ))
        );
        Ok(JsValue::from_f64(documents.len() as f64))
    }

    async fn close(&mut self) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::close",
            &JsValue::from_str("Close operation called.")
        );
        let mut index_guard = self.by_index.write()
            .map_err(|_| {
                let msg = "Failed to acquire write lock".to_string();
                Logger::log(
                    "InMemory::close",
                    &JsValue::from_str(&format!("Error: {}", msg))
                );
                JsValue::from_str(&msg)
            })?;
        index_guard.clear();

        self.started = false;

        Logger::log(
            "InMemory::close",
            &JsValue::from_str("In-memory database closed and reset.")
        );
        Ok(JsValue::from_str("In-memory database closed and reset"))
    }

    async fn start(&mut self) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::start",
            &JsValue::from_str("Start operation called.")
        );
        if self.started {
            Logger::log(
                "InMemory::start",
                &JsValue::from_str("In-memory database already started.")
            );
            return Ok(JsValue::from_str("In-memory database already started"));
        }

        self.started = true;

        Logger::log(
            "InMemory::start",
            &JsValue::from_str("In-memory database started.")
        );
        Ok(JsValue::from_str("In-memory database started"))
    }
}


#[wasm_bindgen]
impl InMemory {

    /// Shared logic to gather all documents matching a given query from a specific collection.
    /// This version retrieves document IDs from each matching index and intersects them before
    /// performing a final document filter based on the full query.
    async fn get_matching_documents(
        &self,
        collection_name: &str,
        query: &Query
    ) -> Result<Vec<JsValue>, JsValue> {
        Logger::log(
            "InMemory::get_matching_documents",
            &JsValue::from_str(&format!(
                "Called with collection_name='{}', query={:?}",
                collection_name,
                query
            ))
        );

        let read_lock = self.by_index.read()
            .map_err(|_| JsValue::from_str("Failed to acquire read lock"))?;

        let schema = self.base.schemas
            .get(collection_name)
            .ok_or_else(|| {
                let msg = format!("Collection '{}' not found", collection_name);
                Logger::log(
                    "InMemory::get_matching_documents",
                    &JsValue::from_str(&format!("Error: {}", msg))
                );
                JsValue::from(msg)
            })?;

        let primary_key = schema.primary_key.clone();
        let mut matched_docs = Vec::new();

        // Identify any query properties that have an index
        let query_properties_with_index: Vec<String> = query
            .clone()
            .get_properties()?
            .into_iter()
            .filter(|key| {
                let index_table = format!("idx_{}_{}", collection_name, key);
                read_lock.get(&index_table).is_some()
            })
            .collect();

        Logger::log(
            "InMemory::get_matching_documents",
            &JsValue::from_str(&format!(
                "Indexed properties in use: {:?}",
                query_properties_with_index
            ))
        );

        // If no indexed fields match the query, do a full table scan.
        // Otherwise, gather an intersection of document IDs from all relevant indexes.
        if query_properties_with_index.is_empty() {
            Logger::log(
                "InMemory::get_matching_documents",
                &JsValue::from_str("No indexed fields match. Performing full table scan.")
            );
            let table_name = format!("pk_{}_{}", collection_name, primary_key);
            if let Some(documents) = read_lock.get(&table_name) {
                for (_, document) in documents.iter() {
                    let matches = self.core.document_matches_query(document, query)?;
                    if matches {
                        matched_docs.push(document.clone());
                    }
                }
            }
        } else {
            // For each indexed property, gather a set of all doc IDs that match that single index.
            let mut doc_id_sets: Vec<HashSet<String>> = Vec::new();

            for indexed_property in &query_properties_with_index {
                let index_table_name = format!("idx_{}_{}", collection_name, indexed_property);
                let mut this_index_doc_ids = HashSet::new();

                if let Some(index_document) = read_lock.get(&index_table_name) {
                    // For each index entry, gather the relevant document primary keys
                    for idx_value in index_document.values() {
                        let index_items = Reflect::get(idx_value, &JsValue::from_str("items"))
                            .unwrap_or_else(|_| JsValue::from(Array::new()));
                        let document_ids = Array::from(&index_items);

                        for document_id_js in document_ids.iter() {
                            if let Some(document_id_str) = document_id_js.as_string() {
                                this_index_doc_ids.insert(document_id_str);
                            }
                        }
                    }
                }
                doc_id_sets.push(this_index_doc_ids);
            }

            // Intersect all sets to ensure the documents match *all* indexed query parts.
            // If there's only one set, the intersection is just that set.
            let mut intersection_ids = if let Some(first_set) = doc_id_sets.clone().into_iter().next() {
                first_set
            } else {
                HashSet::new()
            };

            // Intersect with the remaining sets
            for set in doc_id_sets.clone().into_iter() {
                intersection_ids = intersection_ids
                    .intersection(&set)
                    .cloned()
                    .collect::<HashSet<String>>();
            }

            // Now fetch the actual documents from the primary-key table
            // and do a final match against the full query conditions.
            let table_name = format!("pk_{}_{}", collection_name, primary_key);
            if let Some(pk_map) = read_lock.get(&table_name) {
                for doc_id in intersection_ids {
                    if let Some(doc) = pk_map.get(&doc_id) {
                        if self.core.document_matches_query(doc, query)? {
                            matched_docs.push(doc.clone());
                        }
                    }
                }
            }
        }

        Logger::log(
            "InMemory::get_matching_documents",
            &JsValue::from_str(&format!(
                "Found {} matching documents.",
                matched_docs.len()
            ))
        );
        
        Ok(matched_docs)
    }
    
    #[wasm_bindgen]
    pub async fn create(name: &str, schemas_js: Object) -> Result<InMemory, JsValue> {
        Logger::log(
            "InMemory::create",
            &JsValue::from_str(&format!(
                "Creating a new InMemory instance with DB name '{}'",
                name
            ))
        );
        let base_res = BaseStorage::new(
            name.to_string(),
            schemas_js,
            None
        );

        match base_res {
            Ok(mut base) => {
                //Adds extra index schemas
                base.add_index_schemas()?;
                Logger::log(
                    "InMemory::create",
                    &JsValue::from_str("Successfully created BaseStorage and added index schemas.")
                );
                Ok(
                    InMemory {
                        base,
                        by_index: RwLock::new(HashMap::new()),
                        core: CoreStorage {},
                        started: false,
                    }
                )
            },
            Err(e) => {
                Logger::log(
                    "InMemory::create",
                    &JsValue::from_str(&format!(
                        "Error creating BaseStorage: {:?}",
                        e
                    ))
                );
                Err(e)
            }
        }
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&self, op: &Operation) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::write_js",
            &JsValue::from_str("write_js called.")
        );
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "find")]
    pub async fn find_js(&self, collection_name: &str, query_js: JsValue) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::find_js",
            &JsValue::from_str(&format!(
                "find_js called for collection '{}'",
                collection_name
            ))
        );
        let schema = self.base.schemas.get(collection_name)
            .ok_or_else(|| JsValue::from( format!("Collection {} not found in find", collection_name)))?;
        let query = Query::new(query_js.clone(), schema.clone())?;
        self.find(collection_name, query.clone()).await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(
        &self,
        collection_name: &str,
        primary_key: JsValue,
    ) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::find_document_by_id_js",
            &JsValue::from_str(&format!(
                "findDocumentById called for collection '{}'",
                collection_name
            ))
        );
        self.find_document_by_id(collection_name, primary_key).await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self, collection_name: &str, query_js: JsValue) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::count_js",
            &JsValue::from_str(&format!(
                "count_js called for collection '{}'",
                collection_name
            ))
        );
        let schema = self.base.schemas.get(collection_name).ok_or_else(|| JsValue::from( format!("Collection {} not found in count", collection_name)))?;
        let query = Query::new(query_js, schema.clone())?;
        self.count(collection_name, query).await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&mut self) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::close_js",
            &JsValue::from_str("close_js called.")
        );
        self.close().await
    }

    #[wasm_bindgen(js_name = "start")]
    pub async fn start_js(&mut self) -> Result<JsValue, JsValue> {
        Logger::log(
            "InMemory::start_js",
            &JsValue::from_str("start_js called.")
        );
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
                    Reflect::set(
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
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from_str("id"))

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
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from_str("1234"))
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
            inmem.write(&create_op).await.unwrap();
        }

        // Test count with query
        let query_value = json_str_to_js_value(r#"{
            "status": "active"
        }"#).unwrap();
        
        let result = inmem.count_js("demo", query_value).await.unwrap();
        assert_eq!(result.as_f64().unwrap(), 2.0);
    }

    #[wasm_bindgen_test(async)]
    async fn test_inmemory_storage_multiple_collections() {
        let schemas_obj = Object::new();
        
        // First collection schema (users)
        let users_schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" }
            }
        }"#;
        let users_schema = json_str_to_js_value(users_schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("users"), &users_schema).unwrap();
        
        // Second collection schema (posts)
        let posts_schema_str = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "title": { "type": "string" },
                "content": { "type": "string" }
            }
        }"#;
        let posts_schema = json_str_to_js_value(posts_schema_str).unwrap();
        Reflect::set(&schemas_obj, &JsValue::from_str("posts"), &posts_schema).unwrap();
        
        let inmem = InMemory::create("test_multi_db", schemas_obj).await.unwrap();

        // Insert data only into users collection
        let user = json_str_to_js_value(r#"{
            "id": "1",
            "name": "Alice",
            "email": "alice@example.com"
        }"#).unwrap();

        let create_op = Operation {
            collection: "users".to_string(),
            op_type: OpType::CREATE,
            data: user,
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from("1"))
        };
        inmem.write(&create_op).await.unwrap();

        // Query the empty posts collection
        let empty_query = json_str_to_js_value("{}").unwrap();
        
        // Test find on empty collection
        let posts_result = inmem.find_js("posts", empty_query.clone()).await.unwrap();
        let posts_array = Array::from(&posts_result);
        assert_eq!(posts_array.length(), 0);
        
        // Test count on empty collection
        let count_result = inmem.count_js("posts", empty_query).await.unwrap();
        assert_eq!(count_result.as_f64().unwrap(), 0.0);
    }

    #[wasm_bindgen_test(async)]
    async fn test_inmemory_storage_reuse_after_close() {
        let schemas_obj = Object::new();
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
        Reflect::set(&schemas_obj, &JsValue::from_str("demo"), &schema).unwrap();

        let mut inmem = InMemory::create("test_db", schemas_obj).await.unwrap();

        // Start the storage
        inmem.start_js().await.unwrap();

        // Perform some operations
        let new_item = json_str_to_js_value(r#"{
            "id": "1", "name": "Test Item"
        }"#).unwrap();
        let op = Operation {
            collection: "demo".to_string(),
            op_type: OpType::CREATE,
            data: new_item,
            primary_key_field: Some("id".to_string()),
            primary_key: Some(JsValue::from("1"))
        };
        inmem.write(&op).await.unwrap();

        // Close the storage
        inmem.close_js().await.unwrap();

        // Start the storage again
        inmem.start_js().await.unwrap();

        // Ensure storage is empty after restart
        let query_value = json_str_to_js_value("{}").unwrap();
        let result = inmem.find_js("demo", query_value).await.unwrap();
        let result_array = Array::from(&result);
        assert_eq!(result_array.length(), 0);
    }
}
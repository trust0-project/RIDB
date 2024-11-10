pub mod base_storage;
pub mod core;

// use js_sys::Reflect;
// use wasm_bindgen::JsValue;

// use crate::error::RIDBError;
// use crate::operation::{OpType, Operation};
// use crate::plugin::BasePlugin;
// use crate::schema::Schema;

// use super::base::StorageInternal;

// #[derive(Clone, Default)]
// /// Represents the internals of a storage system, including schema and storage internal components.
// pub struct Internals {
//     /// The schema of the storage system.
//     pub(crate) schema: Schema,
//     /// The internal storage mechanism.
//     pub(crate) internal: StorageInternal,
//     pub(crate) migration: JsValue,
//     pub(crate) plugins: Vec<BasePlugin>
// }



// impl Internals {
//     /// Creates a new `Internals` instance with the provided internal storage.
//     ///
//     /// # Arguments
//     ///
//     /// * `internal` - The internal storage mechanism.
//     ///
//     /// # Returns
//     ///
//     /// * `Internals` - A new instance of `Internals`.
//     pub(crate) fn new(
//         internal: StorageInternal,
//         migration: JsValue,
//         plugins: Vec<BasePlugin>
//     ) -> Result<Internals, JsValue> {
//         let schema = internal.schema().clone();
//         match schema.is_valid() {
//             Ok(_) => Ok(
//                 Internals {
//                     schema,
//                     internal,
//                     migration,
//                     plugins
//                 }
//             ),
//             Err(e) => Err(JsValue::from(e))
//         }
//     }

    
//     /// Writes a document to the storage system after validating the schema.
//     ///
//     /// # Arguments
//     ///
//     /// * `document_without_pk` - The document to write.
//     ///
//     /// # Returns
//     ///
//     /// * `Result<JsValue, JsValue>` - A result containing the written document or an error.
//     pub(crate) async fn write(&self, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
//         let primary_key = self.schema.primary_key.clone();
//         let document = self.validate_schema(document_without_pk)
//             .map_err(|e| JsValue::from(RIDBError::from(e)))?;

//         let indexes = match self.schema.indexes.clone() {
//             Some(mut existing) => {
//                 existing.push(primary_key.clone());
//                 existing
//             },
//             _ => {
//                 let mut new_index: Vec<String> = Vec::new();
//                 new_index.push(primary_key.clone());
//                 new_index
//             }
//         };

//         let pk = Reflect::get(
//             &document.clone(),
//             &JsValue::from_str(primary_key.as_str())
//         )?;

//         let existing = self.find_document_by_id(
//             &self.internal.name(),
//             pk
//         ).await?;

//         let op = if existing.is_null() {
//             Operation {
//                 collection: self.internal.name().clone(),
//                 op_type: OpType::CREATE,
//                 data: document,
//                 indexes,
//             }
//         } else {
//             Operation {
//                 collection: self.internal.name().clone(),
//                 op_type: OpType::UPDATE,
//                 data: document,
//                 indexes,
//             }
//         };

//         let result = self.internal.write(op).await;
//         result.map_err(|e| JsValue::from(RIDBError::from(e)))
//     }

//     /// Placeholder for querying the storage system.
//     pub(crate) async fn find(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue> {
//         self.internal.find(collection_name, query).await
//     }

//     /// Placeholder for finding a document by its ID.
//     pub(crate) async fn find_document_by_id(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue>{
//         match self.internal.findDocument_by_id(collection_name, primary_key).await {
//             Ok(document) => Ok(document),
//             Err(_) => Ok(JsValue::NULL),
//         }

//     }

//     /// Placeholder for counting documents in the storage system.
//     pub(crate) async fn count(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue> {
//         self.internal.count(collection_name, query).await
//     }

//     /// Placeholder for removing a document from the storage system.
//     pub(crate) async fn remove(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue> {
//         let result = self.find_document_by_id(collection_name, primary_key.clone()).await?;
//         if result.is_null() {
//             Err(JsValue::from_str("Invalid primary key value"))
//         } else {
//             let op = Operation {
//                 collection: self.internal.name().clone(),
//                 op_type: OpType::DELETE,
//                 data: result,
//                 indexes: vec![self.schema.primary_key.clone()],
//             };
//             let result = self.internal.write(op).await;
//             result.map_err(|e| JsValue::from(RIDBError::from(e)))
//         }
//     }

//     /// Placeholder for closing the storage system.
//     pub(crate) async fn close(&self) -> Result<JsValue, JsValue> {
//         self.internal.close().await
//     }
// }

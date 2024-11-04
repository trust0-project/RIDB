pub mod storage_internal;
pub mod base_storage;

use js_sys::{Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast,  JsValue};
use web_sys::console::log_1;
use crate::error::RIDBError;
use crate::operation::{OpType, Operation};
use crate::plugin::BasePlugin;
use crate::schema::property_type::PropertyType;
use crate::schema::Schema;
use crate::storage::internals::storage_internal::StorageInternal;




#[derive(Clone, Default)]
/// Represents the internals of a storage system, including schema and storage internal components.
pub struct Internals {
    /// The schema of the storage system.
    pub(crate) schema: Schema,
    /// The internal storage mechanism.
    pub(crate) internal: StorageInternal,
    pub(crate) migration: JsValue,
    pub(crate) plugins: Vec<BasePlugin>
}

#[derive(Debug)]
pub(crate) enum HookType {
    Create,
    Recover,
}

impl HookType {
    fn as_str(&self) -> &'static str {
        match self {
            HookType::Create => "create",
            HookType::Recover => "recover",
        }
    }
}

impl Internals {
    /// Creates a new `Internals` instance with the provided internal storage.
    ///
    /// # Arguments
    ///
    /// * `internal` - The internal storage mechanism.
    ///
    /// # Returns
    ///
    /// * `Internals` - A new instance of `Internals`.
    pub(crate) fn new(
        internal: StorageInternal,
        migration: JsValue,
        plugins: Vec<BasePlugin>
    ) -> Result<Internals, JsValue> {
        let schema = internal.schema().clone();
        match schema.is_valid() {
            Ok(_) => Ok(
                Internals {
                    schema,
                    internal,
                    migration,
                    plugins
                }
            ),
            Err(e) => Err(JsValue::from(e))
        }
    }

    pub(crate) fn call(&self, hook_type: HookType, mut doc: JsValue) -> Result<JsValue, JsValue> {
        let plugins = &self.plugins;
        for plugin in plugins {
            let hook = match hook_type {
                HookType::Create => plugin.get_doc_create_hook(),
                HookType::Recover => plugin.get_doc_recover_hook(),
            };
            doc = self.compute_hook(doc, &hook)?;
        }
        Ok(doc)
    }

    fn compute_hook(&self, doc: JsValue, hook: &JsValue) -> Result<JsValue, JsValue> {
        if hook.is_function() {
            let hook_fn = hook.dyn_ref::<js_sys::Function>()
                .ok_or_else(|| JsValue::from(RIDBError::error("Hook is not a function")))?;

            let call = hook_fn.call3(
                &JsValue::NULL,
                &to_value(&self.schema)?,
                &self.migration,
                &doc
            );

            call.map_err(|e| JsValue::from(RIDBError::error(&format!("Error executing plugin hook: {:?}", e))))?;
        } else {
            log_1(&JsValue::from(format!("InValid Hook type: {:?}", hook.js_typeof())));
        }
        Ok(doc)
    }

    /// Ensures that the document has a primary key, generating one if necessary.
    ///
    /// # Arguments
    ///
    /// * `document` - The document to ensure a primary key for.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the document with the primary key or an error.
    fn ensure_primary_key(&self, document: JsValue) -> Result<JsValue, JsValue> {
        let key = self.schema.primary_key.clone();
        let doc_property = Reflect::get(&document, &JsValue::from(&key))
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;
        let properties = self.schema.properties.clone();
        let primary_key_property = properties
            .get(&key)
            .ok_or(RIDBError::from("Invalid Schema cannot find primaryKey field"))
            .map_err(|e| JsValue::from(e))?;
        let primary_key_type = primary_key_property.property_type();
        if doc_property.is_null() || doc_property.is_undefined() {
            if primary_key_type == PropertyType::String {
                Reflect::set(&document, &JsValue::from(&key), &JsValue::from("12345"))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))?;
            } else {
                Reflect::set(&document, &JsValue::from(&key), &JsValue::from(12345))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))?;
            }
        }
        let doc_property = Reflect::get(&document, &JsValue::from(&key))
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;
        if primary_key_type == PropertyType::String && !doc_property.is_string() {
            Err(JsValue::from(RIDBError::from("Unexpected primary key should be a string")))
        } else if primary_key_type == PropertyType::Number && !doc_property.is_bigint() {
            Err(JsValue::from(RIDBError::from("Unexpected primary key should be number")))
        } else {
            Ok(document)
        }
    }

    /// Validates a document against the schema, ensuring all required fields and types are correct.
    ///
    /// # Arguments
    ///
    /// * `document_without_pk` - The document to validate.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the validated document or an error.
    pub fn validate_schema(&self, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        let document = self.ensure_primary_key(document_without_pk)?;
        let properties = self.schema.properties.clone();
        let required = self.schema.required.clone().unwrap_or(Vec::new());
        for (key, prop) in properties {
            let value = Reflect::get(&document, &JsValue::from_str(&key))?;
            if value.is_undefined() {
                if required.contains(&key) {
                    return Err(JsValue::from(RIDBError::error(
                        &format!("Field {} is required", key),
                    )));
                }
            } else {
                if !self.is_type_correct(&value, prop.property_type) {
                    return Err(JsValue::from(RIDBError::error(
                        &format!("Field {} should match type {:?}", key, prop.property_type),
                    )));
                }
            }
        }
        Ok(document)
    }

    /// Checks if a value is of the correct type based on the property type.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    /// * `prop_type` - The expected property type.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the value is of the correct type, otherwise `false`.
    pub fn is_type_correct(&self, value: &JsValue, prop_type: PropertyType) -> bool {
        match prop_type {
            PropertyType::String => value.is_string(),
            PropertyType::Number => value.as_f64().is_some(),
            PropertyType::Object => value.is_object(),
            PropertyType::Array => {
                if let Some(array) = value.dyn_ref::<js_sys::Array>() {
                    !array.is_array()
                } else {
                    false
                }
            },
            PropertyType::Boolean => value.is_falsy() || value.is_truthy(),
            _ => false,
        }
    }

    /// Writes a document to the storage system after validating the schema.
    ///
    /// # Arguments
    ///
    /// * `document_without_pk` - The document to write.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the written document or an error.
    pub(crate) async fn write(&self, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        let primary_key = self.schema.primary_key.clone();
        let document = self.validate_schema(document_without_pk)
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;

        let indexes = match self.schema.indexes.clone() {
            Some(mut existing) => {
                existing.push(primary_key.clone());
                existing
            },
            _ => {
                let mut new_index: Vec<String> = Vec::new();
                new_index.push(primary_key.clone());
                new_index
            }
        };

        let pk = Reflect::get(
            &document.clone(),
            &JsValue::from_str(primary_key.as_str())
        )?;

        let existing = self.find_document_by_id(
            pk
        ).await?;

        let op = if existing.is_null() {
            Operation {
                collection: self.internal.name().clone(),
                op_type: OpType::CREATE,
                data: document,
                indexes,
            }
        } else {
            Operation {
                collection: self.internal.name().clone(),
                op_type: OpType::UPDATE,
                data: document,
                indexes,
            }
        };

        let result = self.internal.write(op).await;
        result.map_err(|e| JsValue::from(RIDBError::from(e)))
    }

    /// Placeholder for querying the storage system.
    pub(crate) async fn find(&self, query: JsValue) -> Result<JsValue, JsValue> {
        self.internal.find(query).await
    }

    /// Placeholder for finding a document by its ID.
    pub(crate) async fn find_document_by_id(&self, primary_key: JsValue) -> Result<JsValue, JsValue>{
        match self.internal.findDocument_by_id(primary_key).await {
            Ok(document) => Ok(document),
            Err(_) => Ok(JsValue::NULL),
        }

    }

    /// Placeholder for counting documents in the storage system.
    pub(crate) async fn count(&self, query: JsValue) -> Result<JsValue, JsValue> {
        self.internal.count(query).await
    }

    /// Placeholder for removing a document from the storage system.
    pub(crate) async fn remove(&self, primary_key: JsValue) -> Result<JsValue, JsValue> {
        let result = self.find_document_by_id(primary_key.clone()).await?;
        if result.is_null() {
            Err(JsValue::from_str("Invalid primary key value"))
        } else {
            let op = Operation {
                collection: self.internal.name().clone(),
                op_type: OpType::DELETE,
                data: result,
                indexes: vec![self.schema.primary_key.clone()],
            };
            let result = self.internal.write(op).await;
            result.map_err(|e| JsValue::from(RIDBError::from(e)))
        }
    }

    /// Placeholder for closing the storage system.
    pub(crate) async fn close(&self) -> Result<JsValue, JsValue> {
        self.internal.close().await
    }
}

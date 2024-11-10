use std::collections::HashMap;

use base::StorageExternal;
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue};

use crate::{error::RIDBError, operation::{OpType, Operation}, plugin::BasePlugin, schema::{property_type::PropertyType, Schema}};

pub mod internals;
pub mod base;
pub mod indexdb;
pub mod inmemory;

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

#[derive(Clone)]
/// Represents the storage system containing a map of internal storages.
pub struct Storage {
    /// A map where the key is a string and the value is an instance of `Internals`.
    pub(crate) internal: StorageExternal,
    pub(crate) plugins: Vec<BasePlugin>,
    pub(crate) schemas: HashMap<String, Schema>,
    pub(crate) migrations: HashMap<String, JsValue>
}

impl Storage {
    /// Creates a new `Storage` instance from a JavaScript object.
    ///
    /// # Arguments
    ///
    /// * `storages_map_js` - A JavaScript `Object` representing the storages map.
    ///
    /// # Returns
    ///
    /// * `Result<Storage, JsValue>` - A result containing the new `Storage` instance or an error.
    pub fn create(
        schemas: HashMap<String, Schema>,
        migrations: HashMap<String, JsValue>,
        plugins: Vec<BasePlugin>,
        storage: StorageExternal
    ) -> Result<Storage, JsValue> {
        web_sys::console::log_1(&"Creating new Storage instance".into());
        let storage = Storage {
            internal: storage,
            plugins,
            schemas,
            migrations
        };
        Ok(storage)
    }

    pub fn get_schema(&self, collection_name: &str) -> Result<&Schema, JsValue> {
        self.schemas.get(collection_name)
            .ok_or(
                JsValue::from_str(
                    &format!("Invalid collection {}, not found", collection_name)
                )
            )
            .map(|schema| schema)
    }

    pub fn get_migration(&self, collection_name: &str) -> Result<&JsValue, JsValue> {
        self.migrations.get(collection_name)
            .ok_or(
                JsValue::from_str(
                    &format!("Invalid collection {}, not found", collection_name)
                )
            )
            .map(|migration| migration)
    }

    pub(crate) fn call(&self, collection_name: &str,hook_type: HookType, mut doc: JsValue) -> Result<JsValue, JsValue> {
        let plugins = &self.plugins;
        for plugin in plugins {
            let hook = match hook_type {
                HookType::Create => plugin.get_doc_create_hook(),
                HookType::Recover => plugin.get_doc_recover_hook(),
            };
            doc = self.compute_hook(
                collection_name, 
                doc.clone(), 
                &hook
            )?;
        }
        Ok(doc)
    }

    fn compute_hook(&self, collection_name: &str, doc: JsValue, hook: &JsValue) -> Result<JsValue, JsValue> {
        let schema = self.get_schema(collection_name)?;
        let migration = self.get_migration(collection_name)?;
        if !hook.is_function() {
            return Err(JsValue::from(RIDBError::error("Hook must be a function")));
        }
        let hook_fn = hook.dyn_ref::<js_sys::Function>()
            .ok_or_else(|| JsValue::from(RIDBError::error("Hook is not a function")))?;
        hook_fn.call3(
            &JsValue::NULL,
            &to_value(&schema)?,
            &migration,
            &doc
        ).map_err(|e| JsValue::from(RIDBError::error(&format!("Error executing plugin hook: {:?}", e))))
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
    fn ensure_primary_key(&self, collection_name: &str, document: JsValue) -> Result<JsValue, JsValue> {
        let schema = self.get_schema(collection_name)?;
        let properties = schema.properties.clone();
        let key = schema.primary_key.clone();

        let doc_property = Reflect::get(&document, &JsValue::from(&key))
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;

        let primary_key_property = properties
            .get(&key)
            .ok_or(JsValue::from("Invalid Schema cannot find primaryKey field"))?;

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
            PropertyType::Array => value.is_array(),
            PropertyType::Boolean => value.is_falsy() || value.is_truthy(),
            _ => false,
        }
    }

    pub fn validate_schema(&self, collection_name: &str, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        let document = self.ensure_primary_key(collection_name, document_without_pk)?;
        let schema = self.get_schema(collection_name)?;
        let properties = schema.properties.clone();
        let required = schema.required.clone().unwrap_or(Vec::new());
        let encrypted = schema.encrypted.clone().unwrap_or(Vec::new());

        for (key, prop) in properties {
            let value = Reflect::get(&document, &JsValue::from_str(&key))?;
            if value.is_undefined() {
                if required.contains(&key) && !encrypted.contains(&key) {
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


    pub(crate) async fn write(&self, collection_name: &str, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        web_sys::console::log_2(
            &"Writing document to collection:".into(),
            &collection_name.into()
        );
        web_sys::console::log_2(
            &"Initial document:".into(),
            &document_without_pk
        );
        
        let schema = self.get_schema(collection_name)?;
        web_sys::console::log_1(&"Schema retrieved successfully".into());

        let primary_key = schema.primary_key.clone();
        let indexes = schema.indexes.clone();
        web_sys::console::log_2(
            &"Primary key:".into(),
            &primary_key.clone().into()
        );

        let document = match self.validate_schema(collection_name, document_without_pk) {
            Ok(doc) => {
                web_sys::console::log_2(
                    &"Schema validation successful. Validated document:".into(),
                    &doc
                );
                doc
            },
            Err(e) => {
                web_sys::console::error_1(&"Schema validation failed:".into());
                web_sys::console::error_1(&e);
                return Err(JsValue::from(RIDBError::from(e)));
            }
        };

        let indexes = match indexes {
            Some(mut existing) => {
                existing.push(primary_key.clone());
                web_sys::console::log_1(&"Using existing indexes with primary key added".into());
                existing
            },
            _ => {
                let mut new_index: Vec<String> = Vec::new();
                new_index.push(primary_key.clone());
                web_sys::console::log_1(&"Created new index array with primary key".into());
                new_index
            }
        };

        let pk = match Reflect::get(&document.clone(), &JsValue::from_str(primary_key.as_str())) {
            Ok(val) => {
                web_sys::console::log_2(&"Retrieved primary key value:".into(), &val);
                val
            },
            Err(e) => {
                web_sys::console::error_1(&"Failed to get primary key from document".into());
                return Err(JsValue::from(RIDBError::from(e)));
            }
        };

        web_sys::console::log_1(&format!("Find by id start in colllection:{}", &collection_name).into());

        let existing = self.find_document_by_id(&collection_name, pk.clone()).await?;


        web_sys::console::log_1(&format!("Find by id result in colllection:{} == {:?}", &collection_name, &existing).into());


        let op_type = if existing.is_null() { OpType::CREATE } else { OpType::UPDATE };

        let op = Operation {
            collection: collection_name.to_string(),
            op_type,
            data: document,
            indexes,
        };

        web_sys::console::log_2(
            &"Executing write operation for collection:".into(),
            &op.clone().collection.into()
        );
        
        match self.internal.write(op).await {
            Ok(result) => {
                web_sys::console::log_1(&"Write operation successful".into());
                Ok(result)
            },
            Err(e) => {
                web_sys::console::error_1(&"Write operation failed:".into());
                web_sys::console::error_1(&e);
                Err(JsValue::from(RIDBError::from(e)))
            }
        }
    }

    pub(crate) async fn find_document_by_id(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue>{
        web_sys::console::log_3(
            &"Finding document by ID in collection:".into(),
            &collection_name.into(),
            &primary_key
        );
        match self.internal.find_document_by_id( 
            collection_name, 
            primary_key
        ).await {
            Ok(document) => {
                web_sys::console::log_1(&if document.is_null() { 
                    "Document not found".into() 
                } else { 
                    "Document found".into() 
                });
                Ok(document)
            },
            Err(_) => Ok(JsValue::NULL),
        }
    }

    pub(crate) async fn remove(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue> {
        web_sys::console::log_2(
            &"Removing document from collection:".into(),
            &collection_name.into()
        );
        let result = self.find_document_by_id(collection_name, primary_key.clone()).await?;
        let schema = self.get_schema(collection_name)?;
        if result.is_null() {
            Err(JsValue::from_str("Invalid primary key value"))
        } else {
            let op = Operation {
                collection: self.internal.name().clone(),
                op_type: OpType::DELETE,
                data: result,
                indexes: vec![
                    schema.primary_key.clone()
                ],
            };
            let result = self.internal.write(op).await;
            result.map_err(|e| JsValue::from(RIDBError::from(e)))
        }
    }



}

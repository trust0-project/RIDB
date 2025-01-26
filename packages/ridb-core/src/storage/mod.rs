use std::collections::HashMap;
use js_sys::{Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue};

use crate::{error::RIDBError, operation::{OpType, Operation}, plugin::BasePlugin, schema::{property_type::PropertyType, Schema}, storages::base::StorageExternal};
use crate::logger::Logger;

pub mod internals;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum HookType {
    Create,
    Recover,
}


#[derive(Clone)]
/// Represents the storage system containing a map of internal storages.
pub struct Storage {
    /// A map where the key is a string and the value is an instance of `Internals`.
    pub internal: StorageExternal,
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

    pub(crate) async fn call(&self, collection_name: &str, hook_type: HookType, mut doc: JsValue) -> Result<JsValue, JsValue> {
        // Determine the order of plugins based on the hook type
        let plugins = match hook_type.clone() {
            HookType::Create => self.plugins.clone(),
            HookType::Recover => {
                let mut reversed_plugins = self.plugins.clone();
                reversed_plugins.reverse(); // Reverse the plugins for Recover
                reversed_plugins
            },
        };

        // Iterate over the plugins in the determined order
        for plugin in plugins {
            let hook = match hook_type.clone() {
                HookType::Create => plugin.get_doc_create_hook(),
                HookType::Recover => plugin.get_doc_recover_hook(),
            };
            // Apply the hook to the document
            doc = self.compute_hook(
                collection_name, 
                doc.clone(), 
                &hook
            )?.clone();
            
        }
        Ok(doc)
    }

    fn compute_hook(&self, collection_name: &str, doc: JsValue, hook: &JsValue) -> Result<JsValue, JsValue> {
        // Log the initial state of the document
        let schema = self.get_schema(collection_name)?;
        let migration = self.get_migration(collection_name)?;

        if !hook.is_function() && !hook.is_undefined() {
            return Err(JsValue::from(RIDBError::error("Hook must be a function")));
        }

        if hook.is_undefined() {
            return Ok(doc);
        }

        let hook_fn = hook.dyn_ref::<js_sys::Function>()
            .ok_or_else(|| JsValue::from(RIDBError::error("Hook is not a function")))?;

        let result = hook_fn.call3(
            &JsValue::NULL,
            &to_value(&schema)?,
            &migration,
            &doc.clone()
        );

        result.map_err(|e| JsValue::from(RIDBError::error(&format!("Error executing plugin hook: {:?}", e))))
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

    pub(crate) async fn write(
        &self,
        collection_name: &str,
        document_without_pk: JsValue,
    ) -> Result<JsValue, JsValue> {

        Logger::debug("Storage-Write", &JsValue::from(
            &format!("\n -------------------------------\n\n Starting write operation for collection '{}'", collection_name)
        ));

        // Prepare the schema and primary key
        let schema = self.get_schema(collection_name)?;
        let primary_key = schema.primary_key.clone();

        let document = self.ensure_primary_key(collection_name, document_without_pk)?;
        let pk_value = Reflect::get(
            &document,
            &JsValue::from_str(&primary_key)
        )?;

        let existing_document = self.find_document_by_id(
            collection_name,
            pk_value.clone()
        ).await?;

        let op_type = if existing_document.is_null() || existing_document.is_undefined() {
            OpType::CREATE
        } else {
            OpType::UPDATE
        };

        // Create and perform the main write operation
        let operation = Operation {
            collection: collection_name.to_string(),
            op_type,
            data: document.clone(),
            primary_key_field: Some(primary_key.clone()),
            primary_key: Some(pk_value)
        };

        Logger::debug("Storage-Write",&JsValue::from(
            &format!("Performing main write operation: {:?}", operation)
        ));

        let document = self
            .internal
            .write(operation)
            .await?;

        Logger::debug("Storage-Write",&JsValue::from(
            &format!("Write operation completed successfully for collection '{}' \n -------------------------------\n\n", collection_name)
        ));

        Ok(document)

    }

    pub(crate) async fn find(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue>{
        //TODO: Use indexes for more efficient document finding
        self.internal.find(collection_name, query).await
    }

    pub(crate) async fn count(&self, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue>{
        //TODO: Use indexes for more efficient counting
        self.internal.count(collection_name, query).await
    }

    pub(crate) async fn find_document_by_id(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue>{
        match self.internal.find_document_by_id( 
            collection_name, 
            primary_key
        ).await {
            Ok(document) => Ok(document),
            Err(_) => Ok(JsValue::NULL),
        }
    }

    pub(crate) async fn remove(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, JsValue> {
        Logger::debug("Storage-Remove",&JsValue::from(&format!(
            "Starting remove operation for collection: {}, primary_key: {:?}",
            collection_name, primary_key
        )));

        let result = self.find_document_by_id(collection_name, primary_key.clone()).await?;

        Logger::debug("Storage-Remove",&JsValue::from(&format!(
            "Found document for removal: {:?}",
            result
        )));

        if result.is_undefined() | result.is_null() {
            Logger::debug("Storage-Remove",&JsValue::from(
                "Remove operation failed: Document not found"
            ));
            Err(JsValue::from_str("Invalid primary key value"))
        } else {
            let op = Operation {
                collection: collection_name.to_string(),
                op_type: OpType::DELETE,
                data: primary_key.clone(),
                primary_key: None,
                primary_key_field: None
            };

            Logger::debug("Storage-Remove",&JsValue::from(&format!(
                "DELETE OPERATION {:?} ", op
            )));
            
            let result = self.internal.write(op).await;
            
            match &result {
                Ok(_) => Logger::debug("Storage-Remove",&JsValue::from("Remove operation completed successfully")),
                Err(e) => Logger::debug("Storage-Remove",&JsValue::from(&format!("Remove operation failed: {:?}", e))),
            }

            result.map_err(|e| JsValue::from(RIDBError::from(e)))
        }
    }



}

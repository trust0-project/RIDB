use std::collections::HashMap;
use std::sync::Arc;
use js_sys::{Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue};

use crate::{error::RIDBError, operation::{OpType, Operation}, plugin::BasePlugin, schema::{property_type::SchemaFieldType, Schema}, storages::base::StorageExternal};
use crate::utils::Logger;
use crate::query::options::QueryOptions;

pub mod internals;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum HookType {
    Create,
    Recover,
}

// Inner data structure that will be wrapped in Arc
#[derive(Clone)]
struct StorageInner {
    pub internal: StorageExternal,
    pub(crate) plugins: Vec<BasePlugin>,
    pub(crate) schemas: HashMap<String, Schema>,
    pub(crate) migrations: HashMap<String, JsValue>
}

#[derive(Clone)]
/// Represents the storage system containing a map of internal storages.
pub struct Storage {
    /// The inner data wrapped in Arc for thread-safe reference counting
    inner: Arc<StorageInner>,
}

impl Storage {
    pub fn get_internal(&self) -> &StorageExternal {
        &self.inner.internal
    }
    pub fn get_schemas(&self) -> &HashMap<String, Schema> {
        &self.inner.schemas
    }
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
    ) -> Result<Storage, RIDBError> {
        let inner = StorageInner {
            internal: storage,
            plugins,
            schemas,
            migrations
        };
        
        let storage = Storage {
            inner: Arc::new(inner)
        };
        
        Ok(storage)
    }

    pub fn get_schema(&self, collection_name: &str) -> Result<&Schema, RIDBError> {
        self.inner.schemas.get(collection_name)
            .ok_or(
                RIDBError::error(
                    &format!("Invalid collection {}, not found", collection_name),
                    0
                )
            )
            .map(|schema| schema)
    }

    pub fn get_migration(&self, collection_name: &str) -> Result<&JsValue, RIDBError> {
        self.inner.migrations.get(collection_name)
            .ok_or(
                RIDBError::error(
                    &format!("Invalid collection {}, not found", collection_name),
                    0
                )
            )
            .map(|migration| migration)
    }

    pub(crate) async fn call(&self, collection_name: &str, hook_type: HookType, mut doc: JsValue) -> Result<JsValue, RIDBError> {
        // Determine the order of plugins based on the hook type
        let plugins = match hook_type.clone() {
            HookType::Create => self.inner.plugins.clone(),
            HookType::Recover => {
                let mut reversed_plugins = self.inner.plugins.clone();
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

    fn compute_hook(&self, collection_name: &str, doc: JsValue, hook: &JsValue) -> Result<JsValue, RIDBError> {
        // Log the initial state of the document
        let schema = self.get_schema(collection_name)?;
        let migration = self.get_migration(collection_name)?;

        if !hook.is_function() && !hook.is_undefined() {
            return Err(RIDBError::validation("Hook must be a function", 0));
        }

        if hook.is_undefined() {
            return Ok(doc);
        }

        let hook_fn = hook.dyn_ref::<js_sys::Function>()
            .ok_or_else(|| RIDBError::validation("Hook is not a function", 0))?;

        let result = hook_fn.call3(
            &JsValue::NULL,
            &to_value(&schema)?,
            &migration,
            &doc.clone()
        );

        result.map_err(|e| RIDBError::from(e))
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
    fn ensure_primary_key(&self, collection_name: &str, document: JsValue) -> Result<JsValue, RIDBError> {
        let schema = self.get_schema(collection_name)?;
        let properties = schema.properties.clone();
        let key = schema.primary_key.clone();

        let doc_property = Reflect::get(&document, &JsValue::from(&key))
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;

        let primary_key_property = properties
            .get(&key)
            .ok_or(RIDBError::validation("Invalid Schema cannot find primaryKey field", 0))?;

        let primary_key_type = primary_key_property.property_type();

        if doc_property.is_null() || doc_property.is_undefined() {
            if primary_key_type == SchemaFieldType::String {
                Reflect::set(&document, &JsValue::from(&key), &JsValue::from("12345"))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))?;
            } else {
                Reflect::set(&document, &JsValue::from(&key), &JsValue::from(12345))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))?;
            }
        }

        let doc_property = Reflect::get(&document, &JsValue::from(&key))
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;

        if primary_key_type == SchemaFieldType::String && !doc_property.is_string() {
            Err(RIDBError::validation("Unexpected primary key should be a string", 0))
        } else if primary_key_type == SchemaFieldType::Number && !doc_property.is_bigint() {
            Err(RIDBError::validation("Unexpected primary key should be number", 0))
        } else {
            Ok(document)
        }
    }

    pub(crate) async fn write(
        &self,
        collection_name: &str,
        document_without_pk: JsValue,
    ) -> Result<JsValue, RIDBError> {

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
            .inner
            .internal
            .write(operation)
            .await?;

        Logger::debug("Storage-Write",&JsValue::from(
            &format!("Write operation completed successfully for collection '{}' \n -------------------------------\n\n", collection_name)
        ));

        Ok(document)

    }

    pub(crate) async fn find(&self, collection_name: &str, query: JsValue, options: QueryOptions) -> Result<JsValue, RIDBError>{
        //TODO: Use indexes for more efficient document finding
        self.inner.internal.find(collection_name, query, options).await
    }

    pub(crate) async fn count(&self, collection_name: &str, query: JsValue, options: QueryOptions) -> Result<JsValue, RIDBError>{
        //TODO: Use indexes for more efficient counting
        self.inner.internal.count(collection_name, query, options).await
    }

    pub(crate) async fn find_document_by_id(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, RIDBError>{
        match self.inner.internal.find_document_by_id( 
            collection_name, 
            primary_key
        ).await {
            Ok(document) => Ok(document),
            Err(_) => Ok(JsValue::NULL),
        }
    }

    pub(crate) async fn remove(&self, collection_name: &str, primary_key: JsValue) -> Result<JsValue, RIDBError> {
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
            Err( RIDBError::validation("Invalid primary key value",0))
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
            
            let result = self.inner.internal.write(op).await;
            
            match &result {
                Ok(_) => Logger::debug("Storage-Remove",&JsValue::from("Remove operation completed successfully")),
                Err(e) => Logger::debug("Storage-Remove",&JsValue::from(&format!("Remove operation failed: {:?}", e))),
            }

            result
        }
    }



}

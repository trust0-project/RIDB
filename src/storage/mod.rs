pub mod internals;
pub mod inmemory;
mod base;

use std::collections::HashMap;
use js_sys::{ Object, Reflect};
use wasm_bindgen::{JsValue};
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;
use crate::plugin::BasePlugin;
use crate::storage::internals::{Internals};
use crate::storage::internals::storage_internal::StorageInternal;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
/// Represents the storage system containing a map of internal storages.
pub struct Storage {
    /// A map where the key is a string and the value is an instance of `Internals`.
    pub(crate) internals: HashMap<String, Internals>,
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
    pub fn create(storages_map_js: Object, migrations_map_js: Object, plugins: Vec<BasePlugin>) -> Result<Storage, JsValue> {
        if !storages_map_js.is_object() {
            return Err(JsValue::from(RIDBError::from("Unexpected object")));
        }

        // Retrieve keys from the JavaScript object
        let keys = Object::keys(&storages_map_js.clone()).into_iter();

        // Create a HashMap to store the storage internals
        let mut storages: HashMap<String, StorageInternal> = HashMap::new();
        for key in keys {
            let key_string = key.as_string().unwrap();
            let value = Reflect::get(&storages_map_js.clone(), &key)
                .map_err(|e| JsValue::from(RIDBError::from(e)))?;
            storages.insert(key_string, value.clone().into());
        }

        // Mount the storage internals
        let storages_mounted: HashMap<String, Internals> = storages
            .iter()
            .map(|(name, storage_internal)| {
                let migration = Reflect::get(
                    &migrations_map_js,
                    &JsValue::from(name)
                ).map_err(|e| JsValue::from(RIDBError::from(e))).unwrap();
                (name.clone(), Internals::new(
                    storage_internal.clone(),
                    migration,
                    plugins.clone()
                ).unwrap())
            })
            .collect::<HashMap<String, Internals>>();

        let storage = Storage {
            internals: storages_mounted,
        };

        Ok(storage)
    }
}

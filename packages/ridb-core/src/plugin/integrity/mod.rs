use serde_json::{to_string, Value};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::plugin::BasePlugin;
use js_sys::{Object, Reflect, JSON};
use sha3::{Digest, Sha3_512};
use crate::error::RIDBError;

fn sort_json(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                sorted_map.insert(key.clone(), sort_json(map.get(&key).unwrap().clone()));
            }
            Value::Object(sorted_map)
        },
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(sort_json).collect())
        },
        other => other,
    }
}


#[derive(Clone)]
pub struct IntegrityPlugin {
    pub(crate) base: BasePlugin,
}

impl IntegrityPlugin {

    pub(crate) fn new() -> Result<IntegrityPlugin, RIDBError> {
        let base = BasePlugin::new("Integrity".to_string())?;
        let plugin = IntegrityPlugin {
            base,
        };
        let plugin_clone1 = plugin.clone();
        let plugin_clone2 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |_schema, _migration, document| {
            plugin_clone1.clone().add_integrity(document)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, RIDBError>>);

        let recover_hook = Closure::wrap(Box::new(move |_schema, _migration, document| {
            plugin_clone2.clone().check_integrity(document)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, RIDBError>>);

        let mut plugin = plugin;
        plugin.base.doc_create_hook = create_hook.into_js_value();
        plugin.base.doc_recover_hook = recover_hook.into_js_value();
        Ok(plugin)
    }
    

    pub(crate) fn add_integrity(&self,  document: JsValue) -> Result<JsValue, RIDBError> {

        let document_without_integrity = document.clone();
        Reflect::delete_property(&Object::from(document_without_integrity.clone()), &JsValue::from("__integrity"))?;

        // Convert JsValue to serde_json::Value
        let js_string = JSON::stringify(&document_without_integrity)?;
        let serde_value: Value = serde_json::from_str(&js_string.as_string().unwrap())
            .map_err(|e| RIDBError::from(format!("Error converting to serde_json::Value: {:?}", e)))?;

        // Sort the serde_json::Value
        let sorted_value = sort_json(serde_value);

        // Serialize the sorted Value
        let upgraded_str = to_string(&sorted_value)
            .map_err(|e| RIDBError::from(format!("Error serializing sorted JSON: {:?}", e)))?;

        // Compute the hash
        let mut hasher = Sha3_512::new();
        hasher.update(upgraded_str.as_bytes());
        let result = hasher.finalize();
        let hex_hash = hex::encode(result);
       
        // Set the "__integrity" field
        Reflect::set(&document, &JsValue::from("__integrity"), &JsValue::from(hex_hash))?;
        
        Ok(document)
    }


    fn safe_json_copy(&self, document: JsValue) -> JsValue {
        let js_string = JSON::stringify(&document).unwrap();
        let js_string_str = js_string.as_string().unwrap();
        let js_value = JSON::parse(&js_string_str).unwrap();
        js_value
    }

    pub(crate) fn check_integrity(&self, document: JsValue) -> Result<JsValue, RIDBError> {
        let integrity = Reflect::get(&document.clone(), &JsValue::from("__integrity"))?;
        let integrity_str = integrity
            .as_string()
            .ok_or_else(|| RIDBError::from("Error retrieving integrity value"))?;

        // Remove the "__integrity" field from the document
        let document_without_integrity = Object::from(self.safe_json_copy(document.clone()));
        Reflect::delete_property(&document_without_integrity, &JsValue::from("__integrity"))?;
        let js_string = JSON::stringify(&document_without_integrity)?;
        let serde_value: Value = serde_json::from_str(&js_string.as_string().unwrap())
            .map_err(|e| RIDBError::from(format!("Error converting to serde_json::Value: {:?}", e)))?;

        // Sort the serde_json::Value
        let sorted_value = sort_json(serde_value);

        // Serialize the sorted Value
        let upgraded_str = to_string(&sorted_value)
            .map_err(|e| RIDBError::from(format!("Error serializing sorted JSON: {:?}", e)))?;

        // Compute the hash
        let mut hasher = Sha3_512::new();
        hasher.update(upgraded_str.as_bytes());
        let result = hasher.finalize();
        let hex_hash = hex::encode(result);

        if hex_hash != integrity_str {
            return Err(RIDBError::from("Integrity check failed").into());
        }
        Ok(document)
    }

   
}

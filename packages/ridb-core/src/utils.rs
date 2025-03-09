
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use crate::error::RIDBError;

pub fn extract_property<T>(js_value: &JsValue, key: &str) -> Result<T, RIDBError>
    where
        T: for<'de> serde::Deserialize<'de>,
{
    let prop: JsValue = if js_value.is_object() {
        js_sys::Reflect::get(js_value, &JsValue::from(key)).expect("Error getting property")
    } else {
        JsValue::from(js_value)
    };
    from_value(prop).map_err(|err| RIDBError::error(err.to_string().as_str(), 0))
}


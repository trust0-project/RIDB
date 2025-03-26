use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::{ plugin::BasePlugin};
use js_sys::Reflect;
use crate::error::RIDBError;

#[derive(Clone)]
pub struct TimestampPlugin {
    pub(crate) base: BasePlugin,
}

impl TimestampPlugin {

    pub(crate) fn new() -> Result<TimestampPlugin, RIDBError> {
        let base = BasePlugin::new("Timestamp".to_string())?;
        let plugin = TimestampPlugin { base };

        let plugin_clone1 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            plugin_clone1.clone().add_timestamp(schema, migration, content)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, RIDBError>>);

        let plugin = plugin;
        plugin.base.set_doc_create_hook(create_hook.into_js_value());
        Ok(plugin)
    }

    pub(crate) fn add_timestamp(
        &self,
        _schema_js: JsValue,
        _migration: JsValue,
        content: JsValue,
    ) -> Result<JsValue, RIDBError> {
        // Handle both single object and array of objects
        let current_time = js_sys::Date::new_0();
        let unix_timestamp = current_time.get_time() / 1000.0; // Convert to seconds as f64
        
        if content.is_object() {
            let  obj = js_sys::Object::from(content);
            
            // Add createdAt if not present
            if !Reflect::has(&obj, &JsValue::from("createdAt")).unwrap_or(false) {
                Reflect::set(
                    &obj,
                    &JsValue::from("createdAt"),
                    &JsValue::from(unix_timestamp),
                )?;
            }
            
            // Always update updatedAt
            Reflect::set(
                &obj,
                &JsValue::from("updatedAt"),
                &JsValue::from(unix_timestamp),
            )?;
            
            Ok(obj.into())
        } else {
            Ok(content)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::JSON;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_add_timestamp_single_object() {
        // Create test content
        let content_js = r#"{
            "id": "123",
            "name": "test"
        }"#;
        let content = JSON::parse(content_js).unwrap();

        // Test timestamp addition
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content).unwrap();

        // Verify timestamp fields were added
        let result_obj = js_sys::Object::from(result);
        assert!(Reflect::has(&result_obj, &JsValue::from("createdAt")).unwrap());
        assert!(Reflect::has(&result_obj, &JsValue::from("updatedAt")).unwrap());

        // Verify timestamps are numbers
        let created_at = Reflect::get(&result_obj, &JsValue::from("createdAt")).unwrap();
        let updated_at = Reflect::get(&result_obj, &JsValue::from("updatedAt")).unwrap();
        assert!(created_at.as_f64().is_some());
        assert!(updated_at.as_f64().is_some());

        // Verify original fields are preserved
        assert_eq!(
            Reflect::get(&result_obj, &JsValue::from("id"))
                .unwrap()
                .as_string()
                .unwrap(),
            "123"
        );
        assert_eq!(
            Reflect::get(&result_obj, &JsValue::from("name"))
                .unwrap()
                .as_string()
                .unwrap(),
            "test"
        );
    }

    #[wasm_bindgen_test]
    fn test_add_timestamp_existing_timestamps() {
        // Create test content with existing timestamps
        let content_js = r#"{
            "id": "123",
            "createdAt": 1672531200,
            "updatedAt": 1672531200
        }"#;
        let content = JSON::parse(content_js).unwrap();

        // Test timestamp addition
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content).unwrap();

        // Verify existing createdAt was preserved
        let result_obj = js_sys::Object::from(result);
        assert_eq!(
            Reflect::get(&result_obj, &JsValue::from("createdAt"))
                .unwrap()
                .as_f64()
                .unwrap(),
            1672531200.0
        );

        // Verify updatedAt was updated and is a number
        let updated_at = Reflect::get(&result_obj, &JsValue::from("updatedAt")).unwrap();
        assert!(updated_at.as_f64().is_some());
    }

    #[wasm_bindgen_test]
    fn test_add_timestamp_non_object() {
        // Test with non-object content
        let content = JsValue::from_str("test string");
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content).unwrap();

        // Verify content remains unchanged
        assert_eq!(result.as_string().unwrap(), "test string");
    }

    #[wasm_bindgen_test]
    fn test_add_timestamp_null() {
        // Test with null content
        let content = JsValue::NULL;
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content).unwrap();

        // Verify null remains unchanged
        assert!(result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_add_timestamp_undefined() {
        // Test with undefined content
        let content = JsValue::UNDEFINED;
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content).unwrap();

        // Verify undefined remains unchanged
        assert!(result.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_add_timestamp_empty_object() {
        // Test with empty object
        let content = js_sys::Object::new();
        let plugin = TimestampPlugin::new().unwrap();
        let result = plugin.add_timestamp(JsValue::NULL, JsValue::NULL, content.into()).unwrap();

        // Verify timestamp fields were added as numbers
        let result_obj = js_sys::Object::from(result);
        let created_at = Reflect::get(&result_obj, &JsValue::from("createdAt")).unwrap();
        let updated_at = Reflect::get(&result_obj, &JsValue::from("updatedAt")).unwrap();
        assert!(created_at.as_f64().is_some());
        assert!(updated_at.as_f64().is_some());
    }
}

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::{logger::Logger, plugin::BasePlugin, schema::Schema};
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;


#[derive(Clone)]
pub struct DefaultsPlugin {
    pub(crate) base: BasePlugin,
}

impl DefaultsPlugin {

    pub(crate) fn new() -> Result<DefaultsPlugin, JsValue> {
        let base = BasePlugin::new("Defaults".to_string())?;
        let plugin = DefaultsPlugin {
            base,
        };
        let plugin_clone1 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |schema, _migration, document| {
            // Add logging for debugging
            Logger::debug("DefaultsPlugin", &"Creating document with defaults".into());
            let result = plugin_clone1.clone().add_defaults(schema, document);
            if result.is_ok() {
                Logger::debug("DefaultsPlugin", &"Document created successfully".into());
            } else {
                Logger::debug("DefaultsPlugin", &"Failed to create document".into());
            }
            result
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, JsValue>>);
        let mut plugin = plugin;
        plugin.base.doc_create_hook = create_hook.into_js_value();
        Ok(plugin)
    }
    

    pub(crate) fn add_defaults(&self, schema: JsValue, document: JsValue) -> Result<JsValue, JsValue> {
        Logger::debug("DefaultsPlugin", &"Adding defaults to document".into());
        let schema = Schema::create(schema)?;

        let properties = schema.properties.clone();
        for (key, prop) in properties {
            let current_value = Reflect::get(&document, &JsValue::from_str(&key))?;
            if current_value.is_null() || current_value.is_undefined() {
                let has_default = prop.default.is_some();
                if has_default {
                    Logger::debug("DefaultsPlugin",&format!("Setting default for key: {}", key).into());
                    Reflect::set(
                        &document, 
                        &JsValue::from_str(&key), 
                        &to_value(&prop.default)?
                    )?;
                }
            }
        }
        Logger::debug("DefaultsPlugin",&"Defaults added successfully".into());
        Ok(document)
    }

}

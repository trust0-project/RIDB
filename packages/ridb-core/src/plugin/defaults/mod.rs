use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::{ plugin::BasePlugin, schema::Schema, utils::Logger};
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;
use crate::error::RIDBError;

#[derive(Clone)]
pub struct DefaultsPlugin {
    pub(crate) base: BasePlugin,
}

impl DefaultsPlugin {

    pub(crate) fn new() -> Result<DefaultsPlugin, RIDBError> {
        let base = BasePlugin::new("Defaults".to_string())?;
        let plugin = DefaultsPlugin {
            base,
        };
        let plugin_clone1 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |schema, _migration, document| {
            plugin_clone1.clone().add_defaults(schema, document)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, RIDBError>>);
        let plugin = plugin;
        plugin.base.set_doc_create_hook(create_hook.into_js_value());
        Ok(plugin)
    }
    

    pub(crate) fn add_defaults(&self, schema: JsValue, document: JsValue) -> Result<JsValue, RIDBError> {
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

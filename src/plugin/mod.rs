mod encryption;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, WasmAbi};
use js_sys::Object;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

type Hook = (schema: Schema<SchemaType>, doc: Doc<SchemaType>) => Doc<SchemaType>

type BasePluginOptions = {
    schemeCreateHook?: Hook,
    schemaRecoverHook?: Hook
}

export  class BasePlugin {

    /**
     * Frees the resources used by the plugin.
     */
    free(): void;

    docCreateHook?:(schema: Schema<SchemaType>, doc: Doc<SchemaType>) => Doc<SchemaType>;
    docRecoverHook?:(schema: Schema<SchemaType>, doc: Doc<SchemaType>) => Doc<SchemaType>;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct BasePlugin {
    pub(crate) doc_create_hook: JsValue,
    pub(crate) doc_recover_hook: JsValue
}

#[wasm_bindgen]
impl BasePlugin {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<BasePlugin, JsValue> {
        Ok(BasePlugin {
            doc_create_hook: JsValue::undefined(),
            doc_recover_hook: JsValue::undefined(),
        })
    }

    #[wasm_bindgen( getter = docCreateHook)]
    pub fn get_doc_create_hook(&self) -> JsValue {
        self.clone().doc_create_hook
    }

    #[wasm_bindgen( getter = docRecoverHook)]
    pub fn get_doc_recover_hook(&self) -> JsValue {
        self.clone().doc_recover_hook
    }

    #[wasm_bindgen(setter = docCreateHook)]
    pub fn set_doc_create_hook(&mut self, hook: JsValue)  {
        self.doc_create_hook = hook;
    }

    #[wasm_bindgen( setter = docRecoverHook)]
    pub fn set_doc_recover_hook(&mut self, hook: JsValue) {
        self.doc_recover_hook = hook
    }

}

impl From<JsValue> for BasePlugin {
    fn from(js: JsValue) -> Self {
        js.unchecked_into()
    }
}

impl AsRef<JsValue> for BasePlugin {
    fn as_ref(&self) -> &JsValue {
        unsafe { &*(self as *const _ as *const JsValue) }
    }
}

impl JsCast for BasePlugin {
    fn instanceof(val: &JsValue) -> bool {
        val.is_object()
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        BasePlugin {
            doc_create_hook: js_sys::Reflect::get(&val, &JsValue::from_str("docCreateHook"))
                .unwrap_or(JsValue::undefined()),
            doc_recover_hook: js_sys::Reflect::get(&val, &JsValue::from_str("docRecoverHook"))
                .unwrap_or(JsValue::undefined()),
        }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const BasePlugin) }
    }
}

pub mod encryption;
pub mod migration;
pub mod integrity;
pub mod defaults;
pub mod dates;

use std::cell::RefCell;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use js_sys::Reflect;
use crate::error::RIDBError;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
type Hook = (
    schema: Schema<SchemaType>,
    migration: MigrationPathsForSchema<SchemaType>,
    doc: Doc<SchemaType>
) => Doc<SchemaType>

type BasePluginOptions = {
    docCreateHook?: Hook,
    docRecoverHook?: Hook
}

export class BasePlugin implements BasePluginOptions {
     docCreateHook?:Hook;
     docRecoverHook?:Hook;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct BasePlugin {
    pub(crate) doc_create_hook: RefCell<JsValue>,
    pub(crate) doc_recover_hook: RefCell<JsValue>,
    pub(crate) name: String,
}

#[wasm_bindgen]
impl BasePlugin {

    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Result<BasePlugin, RIDBError> {
        Ok(BasePlugin {
            name,
            doc_create_hook: RefCell::new(JsValue::undefined()),
            doc_recover_hook: RefCell::new(JsValue::undefined()),
        })
    }

    #[wasm_bindgen( getter = name)]
    pub fn name(&self) -> JsValue {
       JsValue::from_str(&self.name)
    }

    #[wasm_bindgen( getter = docCreateHook)]
    pub fn get_doc_create_hook(&self) -> JsValue {
        self.doc_create_hook.borrow().clone()
    }

    #[wasm_bindgen( getter = docRecoverHook)]
    pub fn get_doc_recover_hook(&self) -> JsValue {
        self.doc_recover_hook.borrow().clone()
    }

    #[wasm_bindgen(setter = docCreateHook)]
    pub fn set_doc_create_hook(&self, hook: JsValue)  {
        *self.doc_create_hook.borrow_mut() = hook;
    }

    #[wasm_bindgen( setter = docRecoverHook)]
    pub fn set_doc_recover_hook(&self, hook: JsValue) {
        *self.doc_recover_hook.borrow_mut() = hook;
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
            name: "Name".to_string(),
            doc_create_hook: RefCell::new(Reflect::get(&val, &JsValue::from_str("docCreateHook"))
                .unwrap_or(JsValue::undefined())),
            doc_recover_hook: RefCell::new(Reflect::get(&val, &JsValue::from_str("docRecoverHook"))
                .unwrap_or(JsValue::undefined())),
        }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const BasePlugin) }
    }
}

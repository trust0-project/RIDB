use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct QueryOptions {
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>
}



#[wasm_bindgen]
impl QueryOptions {

    #[wasm_bindgen(getter, js_name = "limit")]
    pub fn get_limit(&self) -> Result<JsValue, RIDBError> {
        match self.limit {
            None => Ok(JsValue::undefined()),
            Some(limit) => Ok(JsValue::from(limit))
        }
    }

    #[wasm_bindgen(getter, js_name = "offset")]
    pub fn get_offset(&self) -> Result<JsValue, RIDBError> {
        match self.offset {
            None => Ok(JsValue::undefined()),
            Some(offset) => Ok(JsValue::from(offset))
        }
    }
}
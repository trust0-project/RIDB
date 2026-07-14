use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;

/// The direction used when sorting query results by a given field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SortDirection::Asc => "asc",
            SortDirection::Desc => "desc",
        }
    }
}

/// A single sort instruction: which field to sort by and in which direction.
#[derive(Debug, Clone)]
pub struct SortField {
    pub(crate) field: String,
    pub(crate) direction: SortDirection,
}

#[derive(Debug, Clone, Default)]
#[wasm_bindgen(skip_typescript)]
pub struct QueryOptions {
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
    pub(crate) sort: Vec<SortField>,
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

    /// Exposes the parsed sort specification to JavaScript storage adapters as an
    /// array of `{ field, direction }` objects (empty when no sorting was requested).
    #[wasm_bindgen(getter, js_name = "sort")]
    pub fn get_sort(&self) -> Result<JsValue, RIDBError> {
        let arr = Array::new();
        for field in &self.sort {
            let obj = Object::new();
            Reflect::set(&obj, &JsValue::from_str("field"), &JsValue::from_str(&field.field))?;
            Reflect::set(&obj, &JsValue::from_str("direction"), &JsValue::from_str(field.direction.as_str()))?;
            arr.push(&obj);
        }
        Ok(arr.into())
    }
}

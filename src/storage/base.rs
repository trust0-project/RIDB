use std::format;
use wasm_bindgen::JsValue;
use crate::query::{Query};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::operation::Operation;

pub trait StorageBase {
    async fn write(&mut self, op: &Operation) -> Result<JsValue, JsValue>;
    async fn find(&self, query: Query) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self, primary_key:JsValue) -> Result<JsValue, JsValue>;
    async fn count(&self, query: Query) -> Result<JsValue, JsValue>;
    async fn close(&self) -> Result<JsValue, JsValue>;


}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Default)]
    pub type StorageModule;

    #[wasm_bindgen(method, catch, js_name="createStorage")]
    pub fn create_storage(this: &StorageModule, records: &Object) -> Result<JsValue, JsValue>;
}

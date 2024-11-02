use wasm_bindgen::JsValue;
use crate::query::{Query};
use crate::operation::Operation;

pub trait StorageBase {
    async fn write(&mut self, op: &Operation) -> Result<JsValue, JsValue>;
    async fn find(&self, query: Query) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self, primary_key:JsValue) -> Result<JsValue, JsValue>;
    async fn count(&self, query: Query) -> Result<JsValue, JsValue>;
    async fn close(&self) -> Result<JsValue, JsValue>;
}

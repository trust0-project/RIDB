use js_sys::Object;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::operation::Operation;
use crate::schema::Schema;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents a record of schema types, where each key is a string and the value is a `SchemaType`.
 */
export type SchemaTypeRecord = {
    [name: string]: SchemaType
};

/**
 * Represents a function type for creating storage with the provided schema type records.
 *
 * @template T - The schema type record.
 * @param {T} records - The schema type records.
 * @returns {Promise<InternalsRecord>} A promise that resolves to the created internals record.
 */
export type CreateStorage = <T extends SchemaTypeRecord = SchemaTypeRecord>(
    records: T
) => InternalsRecord;

/**
 * Represents a storage module with a method for creating storage.
 */
export type StorageModule = {
    /**
     * Creates storage with the provided schema type records.
     *
     * @type {CreateStorage}
     */
    createStorage: CreateStorage
};
"#;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Default)]
    pub type StorageInternal;

    #[derive(Clone, Default)]
    pub type StorageModule;

    #[wasm_bindgen(method, catch, js_name="createStorage")]
    pub fn create_storage(this: &StorageModule, records: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(constructor)]
    pub fn new(name: &JsValue, schema: &JsValue) -> StorageInternal;

    #[wasm_bindgen(method, getter = schema)]
    pub fn schema(this: &StorageInternal) -> Schema;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &StorageInternal) -> String;

    #[wasm_bindgen(method, catch)]
    pub async fn write(this: &StorageInternal, op: Operation) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn find(this: &StorageInternal, query: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name="findDocumentById")]
    pub async fn findDocument_by_id(this: &StorageInternal, primary_key:JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn count(this: &StorageInternal, query: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn close(this: &StorageInternal) -> Result<JsValue, JsValue>;
}

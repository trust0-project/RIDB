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
 * Represents the internal storage interface with abstract methods for various storage operations.
 *
 * @template T - The schema type.
 */
export abstract class StorageInternal<T extends SchemaType> {
    /**
     * Writes an operation to the storage.
     *
     * @param {Operation<T>} op - The operation to write.
     * @returns {Promise<Doc<T>>} A promise that resolves to the document written.
     */
    abstract write(op: Operation<T>): Promise<Doc<T>>;

    /**
     * Queries the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the query is complete.
     */
    abstract find(query: QueryType<T>): Promise<Doc<T>[]>;

    /**
     * Finds a document by its ID.
     *
     * @param {string} id - The ID of the document to find.
     * @returns {Promise<null>} A promise that resolves to the found document or null.
     */
    abstract findDocumentById(id: string): Promise<Doc<T> | null>;

    /**
     * Counts the number of documents in the storage.
     *
     * @returns {Promise<number>} A promise that resolves to the number of documents.
     */
    abstract count(query: QueryType<T>): Promise<number>;

    /**
     * Removes a document by its ID.
     *
     * @param {string} id - The ID of the document to remove.
     * @returns {Promise<void>} A promise that resolves when the document is removed.
     */
    abstract remove(id: string): Promise<void>;

    /**
     * Closes the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the storage is closed.
     */
    abstract close(): Promise<void>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Default)]
    pub type StorageInternal;

    #[wasm_bindgen(method, getter = schema)]
    pub fn schema(this: &StorageInternal) -> Schema;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &StorageInternal) -> String;

    #[wasm_bindgen(method, getter = migration)]
    pub fn migration(this: &StorageInternal) -> JsValue;

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

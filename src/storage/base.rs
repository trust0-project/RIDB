use wasm_bindgen::JsValue;
use crate::query::Query;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::operation::Operation;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents a record of schema types, where each key is a string and the value is a `SchemaType`.
 */
export type SchemaTypeRecord = {
    [name: string]: SchemaType
};

export abstract class StorageInternal<Schemas extends SchemaTypeRecord> {
    constructor(
        name: string, 
        schemas: Schemas
    );
    abstract start(): Promise<void>;
    abstract close(): Promise<void>;
    abstract count(
        colectionName: keyof Schemas, 
        query: QueryType<Schemas[keyof Schemas]>
    ): Promise<number>;
    abstract findDocumentById(
        collectionName: keyof Schemas, 
        id: string
    ): Promise<Doc<Schemas[keyof Schemas]> | null>;
    abstract find(
        collectionName: keyof Schemas, 
        query: QueryType<Schemas[keyof Schemas]>
    ): Promise<Doc<Schemas[keyof Schemas]>[]>;
    abstract write(
        op: Operation<Schemas[keyof Schemas]>
    ): Promise<Doc<Schemas[keyof Schemas]>>;
}"#;


//Represents a Storage comming from Javascript
#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Default)]
    pub type StorageExternal;

    #[wasm_bindgen(method, catch)]
    pub async fn write(this: &StorageExternal, op: Operation) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn find(this: &StorageExternal, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name="findDocumentById")]
    pub async fn find_document_by_id(this: &StorageExternal, collection_name: &str, primary_key:JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn count(this: &StorageExternal, collection_name: &str, query: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn close(this: &StorageExternal) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn start(this: &StorageExternal) -> Result<JsValue, JsValue>;
}


//Represents a rust storage
pub trait Storage {
    async fn write(&self, op: &Operation) -> Result<JsValue, JsValue>;
    async fn find(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self, collection_name: &str, primary_key:JsValue) -> Result<JsValue, JsValue>;
    async fn count(&self, collection_name: &str, query: Query) -> Result<JsValue, JsValue>;
    async fn close(&mut self) -> Result<JsValue, JsValue>;
    async fn start(&mut self) -> Result<JsValue, JsValue>;

}

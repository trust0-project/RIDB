use std::collections::HashMap;

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::schema::Schema;

use super::core::CoreStorage;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type BaseStorageOptions =  {
    [name:string]:string | boolean | number
}

export class BaseStorage<Schemas extends SchemaTypeRecord> extends StorageInternal<Schemas> {
    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
        options?: BaseStorageOptions
    ): Promise<
        BaseStorage<
            SchemasCreate
        >
    >;
    
    constructor(
        dbName: string, 
        schemas: Schemas, 
        options?: BaseStorageOptions
    );

    readonly dbName: string;
    readonly schemas: Record<keyof Schemas, Schema<Schemas[keyof Schemas]>>;
    readonly options: BaseStorageOptions;
    readonly core: CoreStorage;
    start(): Promise<void>;
    close(): Promise<void>;
    count(colectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>): Promise<number>;
    findDocumentById(collectionName: keyof Schemas, id: string): Promise<Doc<Schemas[keyof Schemas]> | null>;
    find(collectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>[]>;
    write(op: Operation<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>>;

    getOption(name: string): string | boolean | number | undefined;
    getSchema(name: string): Schema<any>;

}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Debug)]
/// Represents the base storage with a name and schema.
pub struct BaseStorage {
    /// The name of the database.
    pub(crate) name: String,
    /// The schema associated with the storage.
    pub(crate) schemas: HashMap<String, Schema>,
    pub(crate) options: Option<Object>,
    pub(crate) core: CoreStorage,
}

#[wasm_bindgen]
impl BaseStorage {
    /// Creates a new `BaseStorage` instance with the provided name and schema type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the storage.
    /// * `schema_type` - The schema type in `JsValue` format.
    ///
    /// # Returns
    ///
    /// * `Result<BaseStorage, JsValue>` - A result containing the new `BaseStorage` instance or an error.
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, schemas_js: Object, options: Option<Object>) -> Result<BaseStorage, JsValue> {
        let mut schemas: HashMap<String, Schema> = HashMap::new();
        let keys = Object::keys(&schemas_js.clone()).into_iter();
        for collection in keys {
            let collection_string: String = collection.as_string().ok_or("Invalid collection name")?;
            let schema_type = Reflect::get(&schemas_js.clone(), &collection)?;
            let schema = Schema::create(schema_type)?;
            schemas.insert(collection_string.clone(), schema);
        }
        let base_storage = BaseStorage {
            name,
            schemas,
            options,
            core: CoreStorage::new()
        };
        Ok(base_storage)
    }

    #[wasm_bindgen(js_name = getOption)]
    pub fn get_option(&self, name: String) -> Result<JsValue, JsValue> {
        let value = Reflect::get(
            self.options.as_ref().unwrap(), 
            &JsValue::from_str(&name)
        )?;
        Ok(value)
    }

    #[wasm_bindgen(js_name = getSchema)]
    pub fn get_schema(&self, name: String) -> Result<Schema, JsValue> {
        let schema = self.schemas.get(name.as_str()).ok_or_else(|| JsValue::from_str("Schema not found"))?;
        Ok(schema.clone())
    }

    #[wasm_bindgen(getter, js_name = core)]
    pub fn get_core(&self) -> Result<CoreStorage, JsValue> {
        Ok(self.core.clone())
    }

}

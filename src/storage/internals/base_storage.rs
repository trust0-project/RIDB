use std::collections::HashMap;

use js_sys::{Object, Reflect, JSON};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::schema::Schema;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class BaseStorage<Schemas extends SchemaTypeRecord> extends StorageInternal<Schemas> {
    static create<SchemasCreate extends SchemaTypeRecord>(
        name: string,
        schemas: SchemasCreate,
        migrations: Record<
            keyof SchemasCreate, 
            MigrationPathsForSchema<SchemasCreate[keyof SchemasCreate]>
        >,
    ): Promise<
        BaseStorage<
            SchemasCreate
        >
    >;
    
    constructor(
        name: string, 
        schemas: Schemas, 
    );

    readonly name: string;
    readonly schemas: Record<keyof Schemas, Schema<Schemas[keyof Schemas]>>;

    start(): Promise<void>;
    close(): Promise<void>;
    count(colectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>): Promise<number>;
    findDocumentById(collectionName: keyof Schemas, id: string): Promise<Doc<Schemas[keyof Schemas]> | null>;
    find(collectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>[]>;
    write(op: Operation<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>>;

}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
/// Represents the base storage with a name and schema.
pub struct BaseStorage {
    /// The name of the storage.
    pub(crate) name: String,
    /// The schema associated with the storage.
    pub(crate) schemas: HashMap<String, Schema>,
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
    pub fn new(name: String, schemas_js: Object) -> Result<BaseStorage, JsValue> {
        
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
        };
        Ok(base_storage)
    }
}

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
        migrations: Record< keyof Schemas,   MigrationPathsForSchema<Schemas[keyof Schemas]> >
    );

    count(
        collectionName: keyof Schemas, 
        query: QueryType< Schemas[keyof Schemas]  >
    ): Promise<number>;

    findDocumentById(
        collectionName: keyof Schemas, 
        id: string
    ): Promise<Doc<Schemas[keyof Schemas]> | null>;

    find(
        collectionName: keyof Schemas, 
        query: QueryType<Schemas[keyof Schemas]>
    ): Promise<  Doc<Schemas[keyof Schemas]>[]>;

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
    /// The associated schema migration.
    pub(crate) migrations: HashMap<String, JsValue>
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
    pub fn new(name: String, schemas_js: Object, migrations_js: Object) -> Result<BaseStorage, JsValue> {
        web_sys::console::log_1(&format!("Creating new BaseStorage... for db {}", name).into());
        web_sys::console::log_2(&"Name:".into(), &name.clone().into());
        
        let mut schemas: HashMap<String, Schema> = HashMap::new();
        let mut migrations: HashMap<String, JsValue> = HashMap::new();
        let keys = Object::keys(&schemas_js.clone()).into_iter();
        
        web_sys::console::log_1(&"Processing schema collections...".into());
        
        for collection in keys {
            let collection_string: String = collection.as_string().ok_or("Invalid collection name")?;
            web_sys::console::log_2(&"Processing collection:".into(), &collection_string.clone().into());
            
            let schema_type = Reflect::get(&schemas_js.clone(), &collection)?;
            let schema = Schema::create(schema_type)?;
            let migration = Reflect::get(&migrations_js.clone(), &collection)?;
            
            schemas.insert(collection_string.clone(), schema);
            migrations.insert(collection_string, migration);
        }
        
        web_sys::console::log_1(&"BaseStorage creation complete".into());
        
        let base_storage = BaseStorage {
            name,
            schemas,
            migrations
        };
        Ok(base_storage)
    }
}

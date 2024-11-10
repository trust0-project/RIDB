use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{ JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::collection::Collection;
use crate::error::RIDBError;
use crate::plugin::BasePlugin;
use crate::plugin::encryption::EncryptionPlugin;
use crate::plugin::migration::MigrationPlugin;
use crate::schema::Schema;
use crate::storage::base::StorageExternal;
use crate::storage::Storage;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents a database containing collections of documents.
 * RIDB extends from this class and is used to expose collections.
 * 
 * So if you specify:
 * ```typescript
 * const db = new RIDB(
 *     {
 *         schemas: {
 *             demo: {
 *                 version: 0,
 *                 primaryKey: 'id',
 *                 type: SchemaFieldType.object,
 *                 properties: {
 *                     id: {
 *                         type: SchemaFieldType.string,
 *                         maxLength: 60
 *                     }
 *                 }
 *             }
 *         } as const
 *     }
 * )
 * ```
 * 
 * The collection will be available as `db.collections.demo` and all the methods for the collection (find, count, findById, update, create, delete) will be available.
 *
 * @template T - A record of schema types.
 */
export class Database<T extends SchemaTypeRecord> {

    /**
     * Creates a new `Database` instance with the provided schemas and storage module.
     *
     * @template TS - A record of schema types.
     * @param {TS} schemas - The schemas to use for the collections.
     * @param migrations
     * @param plugins
     * @param options
     * @param password
     * @returns {Promise<Database<TS>>} A promise that resolves to the created `Database` instance.
     */
    static create<TS extends SchemaTypeRecord>(
        schemas: TS,
        migrations: MigrationPathsForSchemas<TS> | MigrationPathsForSchema<TS[string]>,
        plugins:Array<typeof BasePlugin>,
        options: RIDBModule,
        password?:string
    ): Promise<Database<TS>>;

    /**
     * The collections in the database.
     *
     * This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.
     */
    readonly collections: {
        [name in keyof T]: Collection<Schema<T[name]>>
    }
}

/**
 * Represents a function type for creating storage with the provided schema type records.
 *
 * @template T - The schema type record.
 * @param {T} records - The schema type records.
 * @returns {Promise<InternalsRecord>} A promise that resolves to the created internals record.
 */
export type CreateStorage = <T extends SchemaTypeRecord>(
    records: T
) => Promise<BaseStorage<T>>;

/**
 * Represents a storage module with a method for creating storage.
 */
export type RIDBModule = {
    /**
     * Creates storage with the provided schema type records.
     *
     * @type {CreateStorage}
     */
    createStorage: CreateStorage,

    /**
     * Plugin constructors array
     */
    apply: (plugins:Array<typeof BasePlugin>) => Array<BasePlugin>;
};
"#;

#[wasm_bindgen]
extern "C" {

    #[derive(Clone, Default)]
    pub type RIDBModule;

    #[wasm_bindgen(method, catch, js_name = "createStorage")]
    pub async fn create_storage(this: &RIDBModule, records: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "apply")]
    pub fn apply(this: &RIDBModule, plugins: Array) -> Result<Vec<JsValue>, JsValue>;

}


#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
/// Represents a database with collections of documents.
pub struct Database {
    /// The storage mechanism for the database.
    pub(crate) storage: Storage
}


#[wasm_bindgen]
impl Database {

    /// Retrieves the collections in the database.
    ///
    /// This function returns an `Object` containing the collections.
    ///
    /// # Returns
    ///
    /// * `Result<Object, JsValue>` - A result containing an `Object` with the collections or an error.
    #[wasm_bindgen(getter)]
    pub fn collections(&self) -> Result<Object, JsValue> {
        let mut collections: HashMap<String, Collection> = HashMap::new();
        for (key, _) in self.storage.schemas.iter() {
            let storage = self.storage.clone();
            let collection = Collection::from(
                key.clone(),
                storage
            );
            collections.insert(
                key.clone(), 
                collection
            );
        }
        let object = Object::new();
        for (key, collection) in collections {
            Reflect::set(
                &object,
                &JsValue::from_str(key.as_str()),
                &JsValue::from(collection)
            ).map_err(|e| JsValue::from(RIDBError::from(e)))?;
        }
        Ok(object)
    }

    #[wasm_bindgen]
    pub async fn create(
        schemas_js: Object,
        migrations_js: Object,
        plugins: Array,
        module: RIDBModule,
        password: Option<String>
    ) -> Result<Database, JsValue> {
        let storage: StorageExternal = module.create_storage(&schemas_js).await?.into();

        let vec_plugins_js: Vec<JsValue> = module.apply(plugins)?;
        let mut vec_plugins: Vec<BasePlugin> = vec_plugins_js.into_iter()
            .map(|plugin| plugin.unchecked_into::<BasePlugin>())
            .collect();

        if let Some(pass) = password {
            let encryption = EncryptionPlugin::new(pass)?;
            vec_plugins.push(encryption.base.clone());
        }

        vec_plugins.push(MigrationPlugin::new()?.base.clone());

        let mut schemas: HashMap<String, Schema> = HashMap::new();
        let mut migrations: HashMap<String, JsValue> = HashMap::new();
        let keys = Object::keys(&schemas_js.clone()).into_iter();
        for collection in keys {
            let collection_string: String = collection.as_string().ok_or("Invalid collection name")?;
            let schema_type = Reflect::get(&schemas_js.clone(), &collection)?;
            let schema = Schema::create(schema_type)?;
            let migration = Reflect::get(&migrations_js.clone(), &collection)?;

            let version = schema.get_version();
            if version > 0 && !migration.is_undefined() {
                let function = Reflect::get(&migration, &JsValue::from(version))
                    .map_err(|e| RIDBError::from(e))?;

                if function.is_undefined() {
                    return Err(
                        JsValue::from(
                            format!("Required Schema {} migration path {} to not be undefined", collection_string, version)
                        )
                    )
                }
            }

            schemas.insert(collection_string.clone(), schema);
            migrations.insert(collection_string, migration);
        }

        let storage = Storage::create(
            schemas,
            migrations,
            vec_plugins,
            storage
        ).map_err(|e| JsValue::from(RIDBError::from(e)))?;

        Ok(Database { storage })
    }
}
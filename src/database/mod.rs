use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{ JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::collection::Collection;
use crate::error::RIDBError;
use crate::plugin::BasePlugin;
use crate::plugin::encryption::EncryptionPlugin;
use crate::plugin::migration::MigrationPlugin;
use crate::storage::Storage;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents a database containing collections of documents.
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
    pub fn create_storage(this: &RIDBModule, records: &Object) -> Result<JsValue, JsValue>;

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
        let  collections: HashMap<String, Collection> =
            self.storage.internals
                .iter()
                .map(|(key, internals)| {
                    (key.clone(), Collection::from(key.clone(), internals.clone()))
                })
                .collect();
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
        schemas_map_js: Object,
        migrations_js: Object,
        plugins: Array,
        module: RIDBModule,
        password: Option<String>
    ) -> Result<Database, JsValue> {
        if !schemas_map_js.is_object() {
            return Err(JsValue::from(RIDBError::from("Unexpected object")));
        }
        let storage_internal_map_js = module.create_storage(
            &schemas_map_js.clone()
        )?;
        let vec_plugins_js: Vec<JsValue> =  module.apply(plugins)?;

        let mut vec_plugins: Vec<BasePlugin> = vec_plugins_js.into_iter()
        .map(|plugin| plugin.unchecked_into::<BasePlugin>())
        .collect();

        if password.is_some() {
            let encryption = EncryptionPlugin::new(password)?;
            vec_plugins.push(encryption.base);
        }

        let migration = MigrationPlugin::new()?;
        vec_plugins.push(migration.base);

        let storage = Storage::create(
            storage_internal_map_js.into(),
            migrations_js,
            vec_plugins,
        ).map_err(|e| JsValue::from(RIDBError::from(e)))?;

        let db = Database {
            storage,
        };

        Ok(db)
    }
}
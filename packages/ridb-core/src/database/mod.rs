use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::collection::Collection;
use crate::error::RIDBError;
use crate::logger::Logger;
use crate::plugin::defaults::DefaultsPlugin;
use crate::plugin::integrity::IntegrityPlugin;
use crate::plugin::BasePlugin;
use crate::plugin::encryption::EncryptionPlugin;
use crate::plugin::migration::MigrationPlugin;
use crate::schema::Schema;
use crate::storage::Storage;
use crate::storages::base::StorageExternal;
use crate::storages::inmemory::InMemory;

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
        db_name: string,
        schemas: TS,
        migrations: MigrationPathsForSchemas<TS> | MigrationPathsForSchema<TS[string]>,
        plugins:Array<typeof BasePlugin>,
        options: RIDBModule,
        password?:string,
        storage?: BaseStorage<TS>
    ): Promise<Database<TS>>;

    authenticate(password: string): Promise<boolean>;

    /**
     * The collections in the database.
     *
     * This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.
     */
    readonly collections: {
        [name in keyof T]: Collection<Schema<T[name]>>
    }

    readonly started: boolean;

    /**
     * Starts the database.
     *
     * @returns {Promise<void>} A promise that resolves when the database is started.
     */
    start(): Promise<void>;

    /**
     * Closes the database.
     *
     * @returns {Promise<void>} A promise that resolves when the database is closed.
     */
    close(): Promise<void>;
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
    pub(crate) storage: Storage,
    pub(crate) started: bool,
    pub(crate) password: Option<String>
}


#[wasm_bindgen]
impl Database {

    #[wasm_bindgen(js_name = "start")]
    pub async fn start(&mut self) -> Result<JsValue, RIDBError> {
        Logger::debug("DB", &"Starting the database...".into());
        if !self.started {
            let res = self.storage.internal.start().await?;
            self.started = true;
            Logger::debug("DB", &"Database started successfully.".into());
            Ok(res)
        } else {
            Ok(JsValue::from_str("Database already started"))
        }
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close(mut self) -> Result<JsValue, RIDBError> {
        Logger::debug("DB",&"Closing the database...".into());
        let res = self.storage.internal.close().await;
        self.started = false;
        Logger::debug("DB",&"Database closed successfully.".into());
        res
    }

    #[wasm_bindgen(getter, js_name = "started")]
    pub fn started(&self) -> bool {
        self.started
    }

    #[wasm_bindgen]
    pub async fn authenticate(&self, password: &str) -> Result<bool, RIDBError> {
        if let Some(stored_password) = &self.password {
            let valid = password == stored_password.as_str();
            match valid {
                true => Ok(true),
                false => Err(RIDBError::authentication("Invalid password", 20))
            }
        } else {
            Ok(false)
        }
    }

    /// Retrieves the collections in the database.
    ///
    /// This function returns an `Object` containing the collections.
    ///
    /// # Returns
    ///
    /// * `Result<Object, JsValue>` - A result containing an `Object` with the collections or an error.
    #[wasm_bindgen(getter)]
    pub fn collections(&self) -> Result<Object, RIDBError> {
        Logger::debug("DB",&"Retrieving collections...".into());
        let object = Object::new();
        for (key, _) in self.storage.schemas.iter() {
            Logger::debug("DB",&format!("Processing collection: {}", key).into());
            let storage = self.storage.clone();
            let collection = Collection::from(
                key.clone(),
                storage
            );
            Reflect::set(
                &object,
                &JsValue::from_str(key.as_str()),
                &JsValue::from(collection)
            ).map_err(|e| JsValue::from(RIDBError::from(e)))?;
        }
        Logger::debug("DB",&"Collections retrieved successfully.".into());
        Ok(object)
    }

    #[wasm_bindgen]
    pub async fn create(
        db_name: &str,
        schemas_js: Object,
        migrations_js: Object,
        plugins: Array,
        module: RIDBModule,
        password: Option<String>,
        storage: Option<StorageExternal>
    ) -> Result<Database, RIDBError> {

        Logger::debug("DB",&format!("Creating database: {}", db_name).into());
        let mut schemas: HashMap<String, Schema> = HashMap::new();
        let mut migrations: HashMap<String, JsValue> = HashMap::new();
        let keys = Object::keys(&schemas_js.clone()).into_iter();

        for collection in keys {
            let collection_string: String = collection.as_string().ok_or("Invalid collection name")?;
            Logger::debug("DB",&format!("Processing schema for collection: {}", collection_string).into());
            let schema_type = Reflect::get(&schemas_js.clone(), &collection)?;
            let schema = Schema::create(schema_type)?;
            let migration = Reflect::get(&migrations_js.clone(), &collection)?;

            let version = schema.get_version();
            if version > 0 && !migration.is_undefined() {
                let function = Reflect::get(&migration, &JsValue::from(version))
                    .map_err(|e| RIDBError::from(e))?;

                if function.is_undefined() {
                    Logger::debug("DB",&format!("Migration path undefined for collection: {}, version: {}", collection_string, version).into());
                    return Err(
                        RIDBError::validation(
                            format!("Required Schema {} migration path {} to not be undefined", collection_string, version).as_str(),
                            20
                        )
                    );
                }
            }

            schemas.insert(collection_string.clone(), schema.clone());
            migrations.insert(collection_string.clone(), migration);
        }


        let storage: StorageExternal = if let Some(storage) = storage {
            Logger::debug("DB",&"Using provided storage.".into());
            storage.into()
        } else {
            Logger::debug("DB",&"Creating InMemory storage.".into());
            JsValue::from(InMemory::create(db_name, schemas_js.clone()).await?).into()
        };

        let vec_plugins_js: Vec<JsValue> = module.apply(plugins)?;
        Logger::debug("DB",&"Plugins applied.".into());
        let mut vec_plugins: Vec<BasePlugin> = vec_plugins_js.into_iter()
            .map(|plugin| plugin.unchecked_into::<BasePlugin>())
            .collect();

        Logger::debug("DB",&"Adding defaults plugin.".into());
        vec_plugins.push(DefaultsPlugin::new()?.base.clone());

        Logger::debug("DB",&"Adding migration plugin.".into());
        vec_plugins.push(MigrationPlugin::new()?.base.clone());

        Logger::debug("DB",&"Adding integrity plugin.".into());
        vec_plugins.push(IntegrityPlugin::new()?.base.clone());

        if let Some(pass) = password.clone() {
            Logger::debug("DB",&"Adding encryption plugin.".into());
            let encryption = EncryptionPlugin::new(pass)?;
            vec_plugins.push(encryption.base.clone());
        }

        Logger::debug("DB",&"Creating storage with schemas and migrations.".into());
        let mounted_storage = Storage::create(
            schemas,
            migrations,
            vec_plugins,
            storage.clone()
        ).map_err(|e| JsValue::from(RIDBError::from(e)))?;

        Logger::debug("DB",&"Database created successfully.".into());
        let db = Database { storage:mounted_storage, started: false, password };
        
        storage.start().await?;

        Ok(db)
    }
}

#[cfg(test)]
mod tests {

    use crate::storages::indexdb::IndexDB;

    use super::*;
    use wasm_bindgen_test::*;
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{prelude::Closure, JsValue};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_database_creation_inmemory() {
        // Create a simple schema
        let schema_js = r#"{
            "users": {
                "version": 0,
                "primaryKey": "id",
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "maxLength": 60
                    },
                    "name": {
                        "type": "string",
                        "maxLength": 100
                    }
                }
            }
        }"#;

        let schemas = js_sys::JSON::parse(schema_js).unwrap();
        let migrations = Object::new();
        let plugins = js_sys::Array::new();
        
        // Create storage module with InMemory storage
        let module = Object::new();
        let create_storage_fn = Closure::wrap(Box::new(move |records: JsValue| {
            wasm_bindgen_futures::future_to_promise(async move {
                let records_obj: Object = records.unchecked_into();
                InMemory::create("test-db", records_obj)
                    .await
                    .map(|storage| JsValue::from(storage))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))
            })
        }) as Box<dyn FnMut(JsValue) -> js_sys::Promise>);
      
        let apply_fn = Function::new_with_args(
            "plugins",
            "return []"
        );
        
        Reflect::set(
            &module,
            &"createStorage".into(), 
            &create_storage_fn.into_js_value()
        ).unwrap();

        Reflect::set(
            &module, 
            &"apply".into(), 
            &apply_fn
        ).unwrap();

        // Create the database
        let mut db = Database::create(
            "test-db",
            schemas.clone().unchecked_into(),
            migrations,
            plugins,
            module.unchecked_into(),
            None,
            None,
        ).await.unwrap();

        // Test that we can get collections
        let collections = db.collections().unwrap();
        assert!(Reflect::has(&collections, &"users".into()).unwrap());

        // Test that we can start the database
        db.start().await.unwrap();

        // Clean up
        db.close().await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn test_database_creation_indexdb() {
        // Create a simple schema
        let schema_js = r#"{
            "users": {
                "version": 0,
                "primaryKey": "id",
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "maxLength": 60
                    },
                    "name": {
                        "type": "string",
                        "maxLength": 100
                    }
                }
            }
        }"#;

        let schemas = js_sys::JSON::parse(schema_js).unwrap();
        let migrations = Object::new();
        let plugins = js_sys::Array::new();
        
        // Create storage module with InMemory storage
        let module = Object::new();
        let create_storage_fn = Closure::wrap(Box::new(move |records: JsValue| {
            wasm_bindgen_futures::future_to_promise(async move {
                let records_obj: Object = records.unchecked_into();
                IndexDB::create("test-db", records_obj)
                    .await
                    .map(|storage| JsValue::from(storage))
                    .map_err(|e| JsValue::from(RIDBError::from(e)))
            })
        }) as Box<dyn FnMut(JsValue) -> js_sys::Promise>);
      
        let apply_fn = Function::new_with_args(
            "plugins",
            "return []"
        );
        
        Reflect::set(
            &module,
            &"createStorage".into(), 
            &create_storage_fn.into_js_value()
        ).unwrap();

        Reflect::set(
            &module, 
            &"apply".into(), 
            &apply_fn
        ).unwrap();

        // Create the database
        let mut db = Database::create(
            "test-db",
            schemas.clone().unchecked_into(),
            migrations,
            plugins,
            module.unchecked_into(),
            None,
            None
        ).await.unwrap();

        // Test that we can get collections
        let collections = db.collections().unwrap();
        assert!(Reflect::has(&collections, &"users".into()).unwrap());

        // Test that we can start the database
        db.start().await.unwrap();

        // Clean up
        db.close().await.unwrap();
    }
      
}
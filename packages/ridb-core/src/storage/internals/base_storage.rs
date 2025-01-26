use std::collections::HashMap;

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::schema::property::Property;
use crate::schema::property_type::PropertyType;
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
    findDocumentById(collectionName: keyof Schemas, id: string): Promise<Doc<Schemas[keyof Schemas]> | null | undefined>;
    find(collectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>[]>;
    write(op: Operation<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>>;
    getOption(name: string): string | boolean | number | undefined;
    getSchema(name: string): Schema<any>;
    //Call addIndexSchemas if you need extra indexing schemas for your database
    addIndexSchemas(): null
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

     fn get_index_schemas(&mut self) -> Result<HashMap<String, Schema>, JsValue> {
        let mut new_schemas: HashMap<String, Schema> = HashMap::new();
        for (collection_name, schema) in self.clone().schemas.into_iter() {
            if schema.indexes.is_some() {
                let indexes = schema.indexes.unwrap();
                for index in indexes {
                    let index_name = format!(
                        "idx_{}_{}",
                        &collection_name,
                        index
                    );
                    let property_type = schema.properties.get(&index).unwrap().property_type;
                    let item_type = schema.properties.get(&schema.primary_key).unwrap().property_type;
                    let empty_vec: Vec<String> = Vec::new();
                    let mut properties:HashMap<String, Property> = HashMap::new();
                    properties.insert(
                        "id".to_string(),
                        Property {
                            property_type,
                            items:None,
                            max_items: None,
                            min_length: None,
                            min_items: None,
                            properties: None,
                            default: None,
                            required: None,
                            max_length: None,
                        }
                    );
                    properties.insert(
                        "items".to_string(),
                        Property {
                            property_type: PropertyType::Array,
                            items:Some(
                                Box::from(
                                    Property {
                                        property_type: item_type,
                                        items: None,
                                        max_items: None,
                                        min_length: None,
                                        min_items: None,
                                        properties: None,
                                        default: None,
                                        required: None,
                                        max_length: None,
                                    }
                                )
                            ),
                            max_items: None,
                            min_length: None,
                            min_items: None,
                            properties: None,
                            default: None,
                            required: None,
                            max_length: None,
                        }
                    );

                    let index_schema = Schema {
                        version: 0,
                        indexes: Some(empty_vec.clone()),
                        encrypted:  Some(empty_vec.clone()),
                        primary_key: "id".to_string(),
                        schema_type: "object".to_string(),
                        properties,
                    };

                    new_schemas.insert(
                        index_name,
                        index_schema
                    );

                }
            }
        }
        Ok(
            new_schemas
        )
    }

    #[wasm_bindgen(js_name = addIndexSchemas)]
    pub fn add_index_schemas(&mut self) -> Result<JsValue, JsValue> {
        let index_schemas = self.get_index_schemas()?;
        for (collection_name, schema) in index_schemas.into_iter() {
            self.schemas.insert(
                collection_name,
                schema
            );
        }
        Ok(JsValue::null())
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

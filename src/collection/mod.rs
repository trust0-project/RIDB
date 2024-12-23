use js_sys::{Object, Reflect, JSON};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::schema::Schema;
use crate::storage::{HookType, Storage};
use sha3::{Digest, Sha3_512};
use web_sys::console;
use serde_json::{Value, to_string};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type InternalsRecord = {
    [name: string]: BaseStorage<SchemaTypeRecord>
};
/**
 * ExtractType is a utility type that maps a string representing a basic data type to the actual TypeScript type.
 *
 * @template T - A string literal type representing the basic data type ('string', 'number', 'boolean', 'object', 'array').
 *
 * @example
 * type StringType = ExtractType<'string'>; // StringType is string
 * type NumberType = ExtractType<'number'>; // NumberType is number
 * type BooleanType = ExtractType<'boolean'>; // BooleanType is boolean
 * type ObjectType = ExtractType<'object'>; // ObjectType is object
 * type ArrayType = ExtractType<'array'>; // ArrayType is Array<any>
 */
export type ExtractType<T extends string> = T extends 'string' ? string :
    T extends 'number' ? number :
    T extends 'boolean' ? boolean :
    T extends 'object' ? object :
    T extends 'array' ? Array<any> :
    never;

/**
 * Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 *
 * type Document = Doc<Schema>; // Document is { name: string; age: number; }
 */
export type Doc<T extends SchemaType> = {
    [name in keyof T['properties']]: ExtractType<T['properties'][name]['type']>
} & {__version?: number };

/**
 * Collection is a class that represents a collection of documents in a database.
 * @template T - A schema type defining the structure of the documents in the collection.
 */
export class Collection<T extends SchemaType> {
    /**
     * Finds all documents in the collection.
     *
     * @returns A promise that resolves to an array of documents.
     */
    find(query: QueryType<T>): Promise<Doc<T>[]>;

    /**
     * count all documents in the collection.
     *
     * @returns A promise that resolves to an array of documents.
     */
    count(query: QueryType<T>): Promise<number>;

    /**
     * Finds a single document in the collection by its ID.
     *
     * @param id - The ID of the document to find.
     * @returns A promise that resolves to the found document.
     */
    findById(id: string): Promise<Doc<T>>;

    /**
     * Updates a document in the collection by its ID.
     *
     * @param id - The ID of the document to update.
     * @param document - A partial document containing the fields to update.
     * @returns A promise that resolves when the update is complete.
     */
    update(document: Partial<Doc<T>>): Promise<void>;

    /**
     * Creates a new document in the collection.
     *
     * @param document - The document to create.
     * @returns A promise that resolves to the created document.
     */
    create(document: Doc<T>): Promise<Doc<T>>;

    /**
     * Deletes a document in the collection by its ID.
     *
     * @param id - The ID of the document to delete.
     * @returns A promise that resolves when the deletion is complete.
     */
    delete(id: string): Promise<void>;
}

"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct Collection {
    pub(crate) name: String,
    pub(crate) storage: Storage,
}

#[wasm_bindgen]
impl Collection {

    /// Constructs a new `Collection` with the given name and internals.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the collection.
    /// * `internals` - Internal storage mechanisms for the collection.
    pub(crate) fn from(
        name: String, 
        storage: Storage
    ) -> Collection {
        Collection {
            name,
            storage,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> Result<Schema, JsValue> {
        let schema = self.storage.get_schema(&self.name)?;
        Ok(
            schema.clone()
        )
    }

    /// Finds and returns all documents in the collection.
    ///
    /// This function is asynchronous and returns a `Schema` representing
    /// the documents found in the collection.
    #[wasm_bindgen]
    pub async fn find(&mut self, query: JsValue) -> Result<JsValue, JsValue> {
        let result = match self.storage.internal.find(&self.name, query).await {
            Ok(docs) => {
                docs
            },
            Err(e) => {
                return Err(js_sys::Error::new(&format!("Failed to find documents: {:?}", e)).into())
            }
        };

        // Convert the result to a JavaScript array
        let array = js_sys::Array::from(&result);
        let processed_array = js_sys::Array::new();

        // Iterate over each document in the array
        for item in array.iter() {
            // Recover the document individually
            let processed_item = self.storage.call(&self.name, HookType::Recover, item.clone()).await?;
            let with_integrity =  self.check_integrity(processed_item.clone())?;
            processed_array.push(&with_integrity);
        }

        Ok(processed_array.into())
    }

    /// counts and returns all documents in the collection.
    ///
    /// This function is asynchronous and returns a `Schema` representing
    /// the documents found in the collection.
    #[wasm_bindgen]
    pub async fn count(&self, query: JsValue) -> Result<JsValue, JsValue> {
        match self.storage.internal.count(&self.name, query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(js_sys::Error::new(&format!("Failed to count documents: {:?}", e)).into())
        }
    }

    /// Finds and returns a single document in the collection by its ID.
    ///
    /// This function is asynchronous.
    #[wasm_bindgen(js_name="findById")]
    pub async fn find_by_id(&self, primary_key: JsValue) -> Result<JsValue, JsValue>{
        let document = match self.storage.internal.find_document_by_id(&self.name, primary_key  ).await {
            Ok(doc) => doc,
            Err(e) => return Err(js_sys::Error::new(&format!("Failed to find document by ID: {:?}", e)).into())
        };
        if document.is_undefined() || document.is_null() {
            Ok(document)
        } else {
            let computed = self.storage.call(
                &self.name, 
                HookType::Recover, 
                document
            ).await?;
            self.check_integrity(computed.clone())
        }
    }

    /// Updates a document in the collection with the given data.
    ///
    /// This function is asynchronous and returns a `Result` indicating success or failure.
    ///
    /// # Arguments
    ///
    /// * `document` - A `JsValue` representing the partial document to update.
    #[wasm_bindgen]
    pub async fn update(&mut self, document: JsValue) -> Result<JsValue, JsValue> {
        let processed_document = self.storage.call(
            &self.name, 
            HookType::Create,
            document
        ).await?;
        
        let valid_doc = self.add_integrity(processed_document.clone())?;
        let res = match self.storage.write(&self.name, valid_doc).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }?;

        self.storage.call(
            &self.name, 
            HookType::Create,
            res
        ).await
    }

    /// Adds an integrity hash to the document.
    fn add_integrity(&self,mut document: JsValue) -> Result<JsValue, JsValue> {
        document = self.storage.set_default_fields(&self.name, document)?;
        console::log_1(
            &JsValue::from(
                format!("Adding integrity to document: {:?}", &document)
            )
        );
        // Remove the "__integrity" field if it exists
        let document_without_integrity = document.clone();
        Reflect::delete_property(&Object::from(document_without_integrity.clone()), &JsValue::from("__integrity"))?;

        // Convert JsValue to serde_json::Value
        let js_string = JSON::stringify(&document_without_integrity)?;
        let serde_value: Value = serde_json::from_str(&js_string.as_string().unwrap())
            .map_err(|e| JsValue::from(format!("Error converting to serde_json::Value: {:?}", e)))?;

        // Sort the serde_json::Value
        let sorted_value = sort_json(serde_value);

        // Serialize the sorted Value
        let upgraded_str = to_string(&sorted_value)
            .map_err(|e| JsValue::from(format!("Error serializing sorted JSON: {:?}", e)))?;

        // Compute the hash
        let mut hasher = Sha3_512::new();
        hasher.update(upgraded_str.as_bytes());
        let result = hasher.finalize();
        let hex_hash = hex::encode(result);
       
        // Set the "__integrity" field
        Reflect::set(&document, &JsValue::from("__integrity"), &JsValue::from(hex_hash))?;
        console::log_1(
            &JsValue::from(
                format!("Integrity added to document: {:?}", &document)
            )
        );
        Ok(document)
    }

    /// Checks the integrity of the document.
    fn check_integrity(&self, document: JsValue) -> Result<JsValue, JsValue> {
        // Get the integrity field
        let integrity = Reflect::get(&document.clone(), &JsValue::from("__integrity"))?;
        let integrity_str = integrity
            .as_string()
            .ok_or_else(|| JsValue::from("Error retrieving integrity value"))?;

        // Remove the "__integrity" field from the document
        let document_without_integrity = document.clone();
        Reflect::delete_property(&Object::from(document_without_integrity.clone()), &JsValue::from("__integrity"))?;

        // Convert JsValue to serde_json::Value

        console::log_1(
            &JsValue::from(
                format!("Checking Integrity to document: {:?}", &document_without_integrity)
            )
        );


        let js_string = JSON::stringify(&document_without_integrity)?;
        let serde_value: Value = serde_json::from_str(&js_string.as_string().unwrap())
            .map_err(|e| JsValue::from(format!("Error converting to serde_json::Value: {:?}", e)))?;

        // Sort the serde_json::Value
        let sorted_value = sort_json(serde_value);

        // Serialize the sorted Value
        let upgraded_str = to_string(&sorted_value)
            .map_err(|e| JsValue::from(format!("Error serializing sorted JSON: {:?}", e)))?;

        // Compute the hash
        let mut hasher = Sha3_512::new();
        hasher.update(upgraded_str.as_bytes());
        let result = hasher.finalize();
        let hex_hash = hex::encode(result);

        if hex_hash != integrity_str {
            return Err(JsValue::from("Integrity check failed"));
        }
        Ok(document)
    }

    /// Creates a new document in the collection.
    ///
    /// This function is asynchronous and returns a `Result` indicating success or failure.
    ///
    /// # Arguments
    ///
    /// * `document` - A `JsValue` representing the document to create.
    #[wasm_bindgen]
    pub async fn create(&mut self, document: JsValue) -> Result<JsValue, JsValue> {
        let processed_document = self.storage.call(
            &self.name, 
            HookType::Create,
            document
        ).await?;

        let with_integrity = self.add_integrity(processed_document)?;

        let res = match self.storage.write(&self.name, with_integrity).await {
            Ok(result) => Ok(result),
            Err(e) =>  Err(e)
        }?;

        self.storage.call(
            &self.name, 
            HookType::Recover,
            res
        ).await
    }

    /// Deletes a document from the collection by its ID.
    ///
    /// This function is asynchronous.
    #[wasm_bindgen]
    pub async fn delete(&self, primary_key: JsValue) -> Result<JsValue, JsValue> {
        match self.storage.remove(&self.name, primary_key ).await {
            Ok(res) => Ok(res),
            Err(e) => Err(js_sys::Error::new(&format!("Failed to delete document: {:?}", e)).into())
        }
    }
}

fn sort_json(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                sorted_map.insert(key.clone(), sort_json(map.get(&key).unwrap().clone()));
            }
            Value::Object(sorted_map)
        },
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(sort_json).collect())
        },
        other => other,
    }
}

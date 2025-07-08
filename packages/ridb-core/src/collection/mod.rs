use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::error::RIDBError;
use crate::query::options::QueryOptions;
use crate::schema::Schema;
use crate::storage::{HookType, Storage};

fn get_u32_option(options: &JsValue, key: &str) -> Result<Option<u32>, JsValue> {
    if options.is_undefined() {
        return Ok(None);
    }

    let value = Reflect::get(options, &JsValue::from_str(key))?;

    // If the value is undefined, we treat it as `None`.
    if value.is_undefined() {
        return Ok(None);
    }

    // If it's a number, convert to u32.
    if let Some(n) = value.as_f64() {
        return Ok(Some(n as u32));
    }

    // Otherwise, we bail out with a descriptive error.
    Err(JsValue::from_str(&format!(
        "Expected '{}' to be undefined or a number.",
        key
    )))
}

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
export type ExtractType<T extends string> = 
  T extends "string" ? string : 
  T extends "number" ? number : 
  T extends "boolean" ? boolean : 
  T extends "object" ? object : 
  T extends "array" ? any[] : 
  undefined;

export type IsOptional<T> = 
  T extends { required: true } 
    ? T extends { default: never } 
      ? false 
      : true
    : true;

/**
 * Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 *
 * type Document = Doc<Schema>; // Document is { name: string; age: number; }
 */
export type Doc<T extends SchemaType> = {
  [K in keyof T["properties"]]: 
    ExtractType<T['properties'][K]['type']>
} & {
  __version?: number;
  createdAt?: number;
  updatedAt?: number;
};

/**
 * CreateDoc is a utility type for document creation that properly handles required vs optional fields
 * during the creation process. Fields with default values or required: false become optional.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 */
export type CreateDoc<T extends SchemaType> = {
  [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? K : never]?: 
    ExtractType<T['properties'][K]['type']>
} & {
  [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? never : K]: 
    ExtractType<T['properties'][K]['type']>
} &  {
  __version?: number;
  createdAt?: number;
  updatedAt?: number;
};

export type QueryOptions = {
    limit?: number;
    offset?: number;
}

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
	find(query: QueryType<T>, options?: QueryOptions): Promise<Doc<T>[]>;
	/**
	 * count all documents in the collection.
	 *
	 * @returns A promise that resolves to an array of documents.
	 */
	count(query: QueryType<T>, options?: QueryOptions): Promise<number>;
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
	create(document: CreateDoc<T>): Promise<Doc<T>>;
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
    pub(crate) storage: Storage
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
            storage
        }
    }

    /// Constructs a new `Collection` using a reference to Storage.
    /// This helps prevent aliasing issues by not cloning Storage for each Collection.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the collection.
    /// * `storage` - A reference to the Storage to be used with this Collection.
    pub(crate) fn with_reference(
        name: String, 
        storage: &Storage
    ) -> Collection {
        // Create a new Collection with a clone of storage, but using
        // Rust's safe reference semantics to prevent aliasing issues
        Collection {
            name,
            storage: storage.clone()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> Result<Schema, RIDBError> {
        let schema = self.storage.get_schema(&self.name)?;
        Ok(
            schema.clone()
        )
    }

    /// Finds and returns all documents in the collection.
    ///
    /// This function is asynchronous and returns a `JsValue` representing
    /// the documents found in the collection.
    #[wasm_bindgen]
    pub async fn find(&self, query_js: JsValue, options_js:JsValue) -> Result<JsValue, RIDBError> {
        let options = self.parse_query_options(options_js)?;

        // Check if both limit and offset are None - if so, use default pagination
        if options.limit.is_none() && options.offset.is_none() {
            return self.load_paginated_results(query_js).await;
        }

        // Use existing logic when limit and/or offset are specified
        let docs = self.storage.find(
            &self.name,
            query_js,
            options.clone()
        ).await?;

        // Process and return the result
        let array = js_sys::Array::from(&docs);
        let  processed_array = js_sys::Array::new();

        // Iterate over each document in the array
        for item in array.iter() {
            // Recover the document individually
            let processed_item = self.storage.call(&self.name, HookType::Recover, item.clone()).await?;
            processed_array.push(&processed_item);
        }

        Ok(
            JsValue::from(
                processed_array
            )
        )
    }

    /// Loads paginated results when no limit/offset is specified.
    /// This implements default pagination with batch size of 20.
    async fn load_paginated_results(&self, query_js: JsValue) -> Result<JsValue, RIDBError> {
        let all_results = js_sys::Array::new();
        let mut current_offset = 0;
        const BATCH_SIZE: u32 = 20;

        loop {
            // Create pagination options for this batch
            let batch_options = QueryOptions {
                limit: Some(BATCH_SIZE),
                offset: Some(current_offset)
            };

            // Fetch the current batch
            let batch_docs = self.storage.find(
                &self.name,
                query_js.clone(),
                batch_options
            ).await?;

            let batch_array = js_sys::Array::from(&batch_docs);
            
            // If the batch is empty, we're done
            if batch_array.length() == 0 {
                break;
            }

            // Process each document in the batch
            for item in batch_array.iter() {
                let processed_item = self.storage.call(&self.name, HookType::Recover, item.clone()).await?;
                all_results.push(&processed_item);
            }

            // Update offset for next batch
            current_offset += batch_array.length();

            // If we got fewer results than the batch size, we're done
            if batch_array.length() < BATCH_SIZE {
                break;
            }
        }

        Ok(JsValue::from(all_results))
    }

    pub fn parse_query_options(&self, options: JsValue) -> Result<QueryOptions, RIDBError> {
        // Use the helper to extract and validate both limit and offset.
        let limit = get_u32_option(&options, "limit")?;
        let offset = get_u32_option(&options, "offset")?;

        Ok(QueryOptions { limit, offset })
    }

    /// counts and returns all documents in the collection.
    ///
    /// This function is asynchronous and returns a `Schema` representing
    /// the documents found in the collection.
    #[wasm_bindgen]
    pub async fn count(&self, query_js: JsValue, options_js:JsValue) -> Result<JsValue, RIDBError> {
        let options = self.parse_query_options(options_js)?;
        self.storage.count(self.name.as_str(), query_js, options.clone()).await
    }

    /// Finds and returns a single document in the collection by its ID.
    ///
    /// This function is asynchronous.
    #[wasm_bindgen(js_name="findById")]
    pub async fn find_by_id(&self, primary_key: JsValue) -> Result<JsValue, RIDBError>{
        let document = self.storage.find_document_by_id(&self.name, primary_key  ).await?;
        if document.is_undefined() || document.is_null() {
            Ok(document)
        } else {
            self.storage.call(
                &self.name, 
                HookType::Recover, 
                document
            ).await
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
    pub async fn update(&self, document: JsValue) -> Result<JsValue, RIDBError> {
        let primary_key = self.schema()?.primary_key;
        let doc_primary_key = Reflect::get(
            &document,
            &JsValue::from(primary_key)
        )?;

        let existing_doc = self.find_by_id(doc_primary_key).await?;
        let merge_docs = JsValue::from(
            Object::assign(
                &Object::from(existing_doc),
                &Object::from(document)
            )
        );

        let processed_document = self.storage.call(
            &self.name, 
            HookType::Create,
            merge_docs
        ).await?;
        
        let res = self.storage.write(&self.name, processed_document).await?;
        self.storage.call(
            &self.name, 
            HookType::Recover,
            res.clone()
        ).await
    }

    /// Creates a new document in the collection.
    ///
    /// This function is asynchronous and returns a `Result` indicating success or failure.
    ///
    /// # Arguments
    ///
    /// * `document` - A `JsValue` representing the document to create.
    #[wasm_bindgen]
    pub async fn create(&self, document: JsValue) -> Result<JsValue, RIDBError> {
        let schema = self.schema()?;
        let processed_document = self.storage.call(
            &self.name, 
            HookType::Create,
            document
        ).await?;
        schema.validate_document(processed_document.clone())?;
        let res = self.storage.write(&self.name, processed_document).await?;
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
    pub async fn delete(&self, primary_key: JsValue) -> Result<JsValue, RIDBError> {
        self.storage.remove(&self.name, primary_key ).await
    }
}

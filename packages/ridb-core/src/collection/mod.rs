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

/**
 * ExtractProperty maps a full Property definition to its document type. Unlike
 * {@link ExtractType} (which only looks at the `type` string), it recurses into
 * `items` for arrays and `properties` for objects, producing precise nested types.
 *
 * @example
 * type Tags = ExtractProperty<{ type: "array"; items: { type: "string" } }>; // string[]
 * type Obj  = ExtractProperty<{ type: "object"; properties: { id: { type: "string" } } }>; // { id: string }
 */
export type ExtractProperty<P> =
  P extends { type: "string" } ? string :
  P extends { type: "number" } ? number :
  P extends { type: "boolean" } ? boolean :
  P extends { type: "array" } ? (P extends { items: infer I } ? ExtractProperty<I>[] : any[]) :
  P extends { type: "object" } ? (P extends { properties: infer PR } ? ExtractObject<PR, NestedRequiredNames<P>> : object) :
  unknown;

/**
 * NestedRequiredNames extracts the union of nested property names listed in an object
 * property's `required` array (JSON Schema semantics), or `never` when no array is
 * present. Note it only matches the array form; a boolean `required` flag yields `never`.
 */
export type NestedRequiredNames<P> =
  P extends { required: infer R }
    ? (R extends readonly string[] ? R[number] : never)
    : never;

/**
 * FlagRequiredness interprets a property's `required` declaration when it is used as a
 * legacy boolean flag. Because `Property.required` is typed `boolean | string[]`, a
 * schema literal written without `as const` widens `true`/`false` to `boolean`; the
 * tuple-wrapped checks below classify each case:
 *  - a literal `false` -> `"optional"`;
 *  - a literal `true`, or a widened `boolean` (whose literal was lost) -> `"required"`.
 *    Treating the ambiguous `boolean` as required matches the Rust validator and turns a
 *    would-be runtime "missing required property" error into a compile-time one instead;
 *  - the array form, or no `required` key -> `"defer"` to the container `required` array.
 */
export type FlagRequiredness<P> =
  P extends { required: infer F }
    ? ([F] extends [readonly string[]] ? "defer"
       : [F] extends [false] ? "optional"
       : [true] extends [F] ? "required"
       : "defer")
    : "defer";

/**
 * IsNestedOptional decides whether a nested property `K` (within an object property's
 * `properties` map `PR`, given that object's required-name union `R`) may be omitted.
 * Precedence mirrors the runtime validator and {@link IsCreateOptional}:
 *  1. a declared `default` makes the field optional;
 *  2. a boolean `required` flag wins (`false` -> optional, `true`/`boolean` -> required);
 *  3. otherwise it is required iff listed in the object's `required` array;
 *  4. otherwise it is optional.
 */
export type IsNestedOptional<PR, K extends keyof PR, R> =
  PR[K] extends { default: unknown } ? true
    : FlagRequiredness<PR[K]> extends "optional" ? true
    : FlagRequiredness<PR[K]> extends "required" ? false
    : K extends R ? false
    : true;

/**
 * ExtractObject builds an object document type from a `properties` map `PR` and the
 * owning object's required-name union `R`, applying the correct optional/required
 * modifier to each nested property (see {@link IsNestedOptional}). This keeps `Doc` and
 * `CreateDoc` in step with the runtime validator, which only enforces nested keys named
 * in that object's `required` array.
 */
export type ExtractObject<PR, R> = {
  [K in keyof PR as IsNestedOptional<PR, K, R> extends true ? never : K]:
    ExtractProperty<PR[K]>
} & {
  [K in keyof PR as IsNestedOptional<PR, K, R> extends true ? K : never]?:
    ExtractProperty<PR[K]>
};

/**
 * The union of property names marked required at the schema level (JSON Schema
 * `required` array). Resolves to `never` when no `required` array is present.
 */
export type RequiredFieldNames<T extends SchemaType> =
  T extends { required: infer R }
    ? (R extends readonly string[] ? R[number] : never)
    : never;

/**
 * IsCreateOptional decides whether a property may be omitted when creating a document.
 * Precedence (mirrors the runtime validator):
 *  1. a declared `default` makes the field optional;
 *  2. a boolean `required` flag wins (`false` -> optional, `true`/`boolean` -> required;
 *     see {@link FlagRequiredness});
 *  3. otherwise it is required iff listed in the schema-level `required` array;
 *  4. otherwise it is optional.
 */
export type IsCreateOptional<T extends SchemaType, K extends keyof T["properties"]> =
  T["properties"][K] extends { default: unknown } ? true
    : FlagRequiredness<T["properties"][K]> extends "optional" ? true
    : FlagRequiredness<T["properties"][K]> extends "required" ? false
    : K extends RequiredFieldNames<T> ? false
    : true;

/**
 * Doc is a utility type that transforms a schema type into a stored-document type. A
 * property is mandatory only when the validator guarantees its presence; properties that
 * are optional at creation (not listed in `required`, flagged `required: false`, or
 * carrying a `default`) may be absent on a stored document, so they are optional here
 * too. This keeps `find`/`findById`/`create` return types from claiming keys that may not
 * exist. Optionality uses the same {@link IsCreateOptional} rules as {@link CreateDoc}.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 *
 * type Document = Doc<Schema>; // Document is { name: string; age: number; }
 */
export type Doc<T extends SchemaType> = {
  [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? never : K]:
    ExtractProperty<T['properties'][K]>
} & {
  [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? K : never]?:
    ExtractProperty<T['properties'][K]>
} & {
  __version?: number;
  createdAt?: number;
  updatedAt?: number;
};

/**
 * CreateDoc is a utility type for document creation that properly handles required vs optional fields
 * during the creation process. Fields with default values, or fields not listed in the schema-level
 * `required` array, become optional.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 */
export type CreateDoc<T extends SchemaType> = {
  [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? K : never]?:
    ExtractProperty<T['properties'][K]>
} & {
  [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? never : K]:
    ExtractProperty<T['properties'][K]>
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

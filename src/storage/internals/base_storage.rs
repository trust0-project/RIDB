use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::schema::Schema;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents the internal storage interface with abstract methods for various storage operations.
 *
 * @template T - The schema type.
 */
export abstract class StorageInternal<T extends SchemaType> {
    /**
     * Writes an operation to the storage.
     *
     * @param {Operation<T>} op - The operation to write.
     * @returns {Promise<Doc<T>>} A promise that resolves to the document written.
     */
    abstract write(op: Operation<T>): Promise<Doc<T>>;

    /**
     * Queries the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the query is complete.
     */
    abstract find(query: QueryType<T>): Promise<Doc<T>[]>;

    /**
     * Finds a document by its ID.
     *
     * @param {string} id - The ID of the document to find.
     * @returns {Promise<null>} A promise that resolves to the found document or null.
     */
    abstract findDocumentById(id: string): Promise<Doc<T> | null>;

    /**
     * Counts the number of documents in the storage.
     *
     * @returns {Promise<number>} A promise that resolves to the number of documents.
     */
    abstract count(query: QueryType<T>): Promise<number>;

    /**
     * Removes a document by its ID.
     *
     * @param {string} id - The ID of the document to remove.
     * @returns {Promise<void>} A promise that resolves when the document is removed.
     */
    abstract remove(id: string): Promise<void>;

    /**
     * Closes the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the storage is closed.
     */
    abstract close(): Promise<void>;
}

/**
 * Represents the base storage implementation, extending `StorageInternal`.
 *
 * @template T - The schema type.
 */
export class BaseStorage<T extends SchemaType> extends StorageInternal<T> {
    /**
     * Frees the resources used by the base storage.
     */
    free(): void;

    /**
     * Creates a new `BaseStorage` instance with the provided name and schema type.
     *
     * @param {string} name - The name of the storage.
     * @param {any} schema_type - The schema type of the storage.
     */
    constructor(name: string, schema_type: any);

    /**
     * The name of the storage.
     */
    readonly name: string;

    /**
     * The schema associated with the storage.
     */
    readonly schema: Schema<T>;

    /**
     * Closes the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the storage is closed.
     */
    close(): Promise<void>;

    /**
     * Counts the number of documents in the storage.
     *
     * @returns {Promise<number>} A promise that resolves to the number of documents.
     */
    count(query: QueryType<T>): Promise<number>;

    /**
     * Finds a document by its ID.
     *
     * @param {string} id - The ID of the document to find.
     * @returns {Promise<null>} A promise that resolves to the found document or null.
     */
    findDocumentById(id: string): Promise<Doc<T> | null>;

    /**
     * Queries the storage.
     *
     * @returns {Promise<void>} A promise that resolves when the query is complete.
     */
    find(query: QueryType<T>): Promise<Doc<T>[]>;

    /**
     * Removes a document by its ID.
     *
     * @param {string} id - The ID of the document to remove.
     * @returns {Promise<void>} A promise that resolves when the document is removed.
     */
    remove(id: string): Promise<void>;

    /**
     * Writes an operation to the storage.
     *
     * @param {Operation<T>} op - The operation to write.
     * @returns {Promise<Doc<T>>} A promise that resolves to the document written.
     */
    write(op: Operation<T>): Promise<Doc<T>>;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
/// Represents the base storage with a name and schema.
pub struct BaseStorage {
    /// The name of the storage.
    pub(crate) name: String,
    /// The schema associated with the storage.
    pub(crate) schema: Schema,
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
    pub fn new(name: String, schema_type: JsValue) -> Result<BaseStorage, JsValue> {
        match Schema::create(schema_type) {
            Ok(schema) => {
                match schema.is_valid() {
                    Ok(_) => {
                        Ok(BaseStorage { name, schema })
                    },
                    Err(e) => {
                        Err(JsValue::from(e))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Retrieves the schema of the storage.
    ///
    /// # Returns
    ///
    /// * `Schema` - The schema associated with the storage.
    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> Schema {
        self.schema.clone()
    }

    /// Retrieves the name of the storage.
    ///
    /// # Returns
    ///
    /// * `String` - The name of the storage.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents an operation to be performed on a collection.
 *
 * @template T - The schema type of the collection.
 */
export type Operation<T extends SchemaType> = {
    /**
     * The name of the collection on which the operation will be performed.
     */
    collection: string,

    /**
     * The type of operation to be performed (e.g., CREATE, UPDATE, DELETE).
     */
    opType: OpType,

    /**
     * The data involved in the operation, conforming to the schema type.
     */
    data: Doc<T>,

    primaryKeyField?: string,
    primaryKey?: string
}
"#;

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
/// Represents an operation to be performed on a collection.
pub struct Operation {
    /// The name of the collection on which the operation will be performed.
    pub(crate) collection: String,
    /// The type of operation (create, update, delete).
    pub(crate) op_type: OpType,
    /// The data involved in the operation.
    pub(crate) data: JsValue,
    /// The primary key field of the current collection
    pub(crate) primary_key_field: Option<String>,
    /// The primary key value of the current data
    pub(crate) primary_key: Option<JsValue>
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
/// Represents the type of operation to be performed on the collection.
pub enum OpType {
    /// Create operation.
    CREATE,
    /// Update operation.
    UPDATE,
    /// Delete operation.
    DELETE,
    /// Query Operation.
    QUERY,
    /// Count Operation.
    COUNT
}

#[wasm_bindgen]
impl Operation {

    /// Retrieves the name of the collection.
    ///
    /// # Returns
    ///
    /// * `String` - The name of the collection.
    #[wasm_bindgen(getter)]
    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    /// Retrieves the type of operation.
    ///
    /// # Returns
    ///
    /// * `OpType` - The type of operation.
    #[wasm_bindgen(getter, js_name="opType")]
    pub fn op_type(&self) -> OpType {
        self.op_type.clone()
    }

    /// Retrieves the data involved in the operation.
    ///
    /// # Returns
    ///
    /// * `JsValue` - The data involved in the operation.
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        self.data.clone()
    }

    /// Retrieves the primary key field of the current collection.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The primary key field of the current collection.
    #[wasm_bindgen(getter, js_name="primaryKeyField")]
    pub fn primary_key_field(&self) -> JsValue {
        let primary_key_field = self.primary_key_field.clone();
        if primary_key_field.is_some() {
            JsValue::from(
                primary_key_field.unwrap()
            )
        } else {
            JsValue::undefined()
        }
    }

    /// Retrieves the primary key value of the current data.
    ///
    /// # Returns
    ///
    /// * `Option<JsValue>` - The primary key value of the current data.
    #[wasm_bindgen(getter, js_name="primaryKey")]
    pub fn primary_key(&self) -> JsValue {
        let primary_key = self.primary_key.clone();
        if primary_key.is_some() {
            JsValue::from(
                primary_key.unwrap()
            )
        } else {
            JsValue::undefined()
        }
    }

    #[wasm_bindgen(getter, js_name="primaryKeyIndex")]
    pub fn primary_key_index(&self) -> Result<String, JsValue> {
        match &self.primary_key_field {
            Some(primary_key_field) => Ok(
                format!(
                    "pk_{}_{}",
                    self.collection, 
                    &primary_key_field
                )
            ),
            None => Err(
                JsValue::from(
                    format!("Unable to create default index, Primary Key not available in current OP")
                )
            ),
        }
    }
}

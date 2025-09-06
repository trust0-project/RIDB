pub mod property_type;
pub mod property;

use std::collections::HashMap;
use js_sys::{Object, Reflect, JSON};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::{ wasm_bindgen_test};
use crate::error::RIDBError;
use crate::schema::property::Property;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents the type definition for a schema.
 */
export type SchemaType = {
    /**
     * The version of the schema.
     */
     version: number;

    /**
     * The primary key of the schema.
     */
     primaryKey: string;

    /**
     * The type of the schema.
     */
     type: SchemaFieldType;
     indexes?:  string[];
     encrypted?:  string[];
    /**
     * The properties defined in the schema.
     */
     properties: {
        [name: string]: Property;
    };
};


/**
 * Represents a schema, including its definition and related methods.
 * You may be trying to build a storage, in any other can u won't need access tho this class.
 * Check this example 
 * 
 * ```typescript
 * class MyStorage extends <T extends SchemaTypeRecord> extends BaseStorage<T> {
 *  example() {
 *    const schema: Schema<any> = this.getSchema("mySchema")
 *  }
 * }
 * ```
 * You alwayswill have access to getSchema through the Storage class.
 * 
 * @template T - The schema type.
 */
export class Schema<T extends SchemaType> {
    /**
     * The schema definition.
     */
    schema: Schema<T>;

    /**
     * Creates a new `Schema` instance from the provided definition.
     *
     * @template TS - The schema type.
     * @param {TS} defi, Debugnition - The schema definition.
     * @returns {Schema<TS>} The created `Schema` instance.
     */
    static create<TS extends SchemaType>(definition: TS): Schema<TS>;

    /**
     * The version of the schema.
     */
    readonly version: number;

    /**
     * The primary key of the schema.
     */
    readonly primaryKey: string;

    /**
     * The type of the schema.
     */
    readonly type: SchemaFieldType;

    /**
     * An optional array of indexes.
     */
    /**
     * An optional array of indexes.
     */
    readonly indexes?: (Extract<keyof T, string>)[];

    /**
     * An optional array of encrypted fields.
     */
    readonly encrypted?: (Extract<keyof T, string>)[];

    /**
     * The properties defined in the schema.
     */
    readonly properties: {
        [K in keyof T['properties'] as T['properties'][K]['required'] extends false | (T['properties'][K]['default'] extends undefined ? true: false)  ? K : never]?: T['properties'][K];
    } & {
        [K in keyof T['properties'] as T['properties'][K]['required'] extends false ? never : K]: T['properties'][K];
    };
    /**
     * Converts the schema to a JSON representation.
     *
     * @returns {SchemaType} The JSON representation of the schema.
     */
    toJSON(): SchemaType;

    validate(document: Doc<Schema<T>>): boolean;
}
"#;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[wasm_bindgen(skip_typescript)]
/// Represents the schema of a collection, including version, primary key, type, required fields, properties, and indexes.
pub struct Schema {
    /// The version of the schema.
    pub(crate) version: i32,
    /// The primary key of the schema.
    #[serde(rename = "primaryKey")]
    pub(crate) primary_key: String,
    /// The type of the schema.
    #[serde(rename = "type")]
    pub(crate) schema_type: String,
    /// The properties defined in the schema.
    pub(crate) properties: HashMap<String, Property>,
    /// The indexes defined in the schema, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) indexes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) encrypted: Option<Vec<String>>
}


#[wasm_bindgen]
impl Schema {

    #[wasm_bindgen(js_name="validate")]
    pub fn validate_document(&self, document: JsValue) -> Result<(), RIDBError> {
        // Collect required fields
        let required: Vec<String> = self.properties
            .iter()
            .filter(|(_, prop)| prop.required.unwrap_or(true))
            .map(|(key, _)| key.clone())
            .collect();

        let encrypted = self.encrypted.clone().unwrap_or(Vec::new());

        for (key, prop) in &self.properties {

            let value = Reflect::get(&document, &JsValue::from_str(&key))
                .map_err(|_e| {
                    JsValue::from(
                        RIDBError::validation(
                            &format!("Missing required property '{}'", key),
                            14
                        )
                    )
                })?;

            if value.is_undefined() {
                // If the property is required and not encrypted, it's an error
                if required.contains(&key) && !encrypted.contains(&key) {
                    return Err(
                            RIDBError::validation(
                                &format!("Missing required property '{}'", key),
                                15
                            )

                    );
                }
            } else {
                let res = self.is_type_correct(&key, &value, &prop);
                if let Err(err) = res {
                    return Err(err);
                } else if !res.unwrap() {
                    return Err(
                            RIDBError::validation(
                                &format!(
                                    "Field '{}' should be of type '{:?}'",
                                    key, prop.property_type
                                ),
                                15
                            )
                    );
                }
            }
        }
        Ok(())
    }


    fn is_type_correct(&self, key: &String, value: &JsValue, property: &Property) -> Result<bool, RIDBError> {
        match property.property_type {
            SchemaFieldType::String => {
                if let Some(string) = value.as_string() {
                    // Check maxLength and minLength if they exist
                    if let Some(max_length) = property.max_length {
                        if string.len() > max_length as usize {
                            return Err(
                                    RIDBError::validation(
                                    &format!( "Property '{}' exceeds maximum length of '{:?}'", key, max_length),
                                    16
                                    )
                            );
                        }
                    }
                    if let Some(min_length) = property.min_length {
                        if string.len() < min_length as usize {
                            return Err(
                                RIDBError::validation(
                                    &format!(
                                        "Property '{}' is lower than min length of '{:?}'",
                                        key, min_length
                                    ), 17)
                            );
                        }
                    }
                    return Ok(true);
                }
                Ok(false)
            },
            SchemaFieldType::Number => {
                // Check if the value can be converted to an f64
                Ok(value.as_f64().is_some())
            },
            SchemaFieldType::Boolean => Ok(value.as_bool().is_some()),
            SchemaFieldType::Object => {
                // Exclude null, arrays, and functions
                Ok(value.is_object()
                    && !value.is_null()
                    && !js_sys::Array::is_array(value))
            },
            SchemaFieldType::Array => {
                let arr = js_sys::Array::from(value);
                if let Some(max_length) = property.max_items {
                    let len_js = arr.length();
                    let length = i32::try_from(len_js).unwrap();
                    if length > max_length  {
                        return Err(
                            RIDBError::validation(
                            &format!(
                                "Property '{}' exceeds maximum items of '{:?}'",
                                key, max_length
                            ), 18)
                        );
                    }
                }
                Ok(true)
            },
            // Add other property types as needed
            _ => Ok(false),
        }
    }
    

    pub fn is_valid(&self) -> Result<bool, RIDBError> {
        // Check if the schema type is "object"
        let schema_type = self.get_schema_type();
        if schema_type != "object" {
            return Err(
                RIDBError::validation(
                    &format!("Schema type is invalid (\"{}\")", schema_type).as_str(),
                    19
                )
            );
        }

        // Validate all properties
        for property in self.properties.values() {
            property.is_valid()?;
        }

        Ok(true)
    }

    /// Creates a new `Schema` instance from a given `JsValue`.
    ///
    /// # Arguments
    ///
    /// * `schema` - A `JsValue` representing the schema.
    ///
    /// # Returns
    ///
    /// * `Result<Schema, JsValue>` - A result containing the new `Schema` instance or an error.
    #[wasm_bindgen]
    pub fn create(schema: JsValue) -> Result<Schema, RIDBError> {
        let schema: Schema = from_value(schema)
            .map_err(|e| JsValue::from(RIDBError::from(e)))?;
        let valid = schema.is_valid();
        match valid {
            Ok(_) =>  Ok(schema),
            Err(e) => Err(e)
        }
    }

    /// Retrieves the version of the schema.
    ///
    /// # Returns
    ///
    /// * `i32` - The version of the schema.
    #[wasm_bindgen(getter, js_name="version")]
    pub fn get_version(&self) -> i32 {
        self.version
    }

    /// Retrieves the primary key of the schema.
    ///
    /// # Returns
    ///
    /// * `String` - The primary key of the schema.
    #[wasm_bindgen(getter, js_name="primaryKey")]
    pub fn get_primary_key(&self) -> String {
        self.primary_key.clone()
    }

    /// Retrieves the type of the schema.
    ///
    /// # Returns
    ///
    /// * `String` - The type of the schema.
    #[wasm_bindgen(getter, js_name="type")]
    pub fn get_schema_type(&self) -> String {
        self.schema_type.clone()
    }

    /// Retrieves the indexes of the schema, if any.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - The indexes of the schema, if any.
    #[wasm_bindgen(getter, js_name="indexes")]
    pub fn get_indexes(&self) -> Option<Vec<String>> {
        self.indexes.clone()
    }

    #[wasm_bindgen(getter, js_name="encrypted")]
    pub fn get_encrypted(&self) -> Option<Vec<String>> {
        self.encrypted.clone()
    }

    /// Retrieves the properties of the schema.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the properties as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name="properties")]
    pub fn get_properties(&self) -> Result<JsValue, RIDBError> {
        // Create a new JavaScript object to hold all properties
        let result = Object::new();

        for (key, property) in &self.properties {
            // Create a JavaScript object for the property
            let prop_obj = Object::new();

            // Get the 'type' field as a string
            let prop_type_str = match property.property_type {
                SchemaFieldType::String => "string",
                SchemaFieldType::Number => "number",
                SchemaFieldType::Boolean => "boolean",
                SchemaFieldType::Array => "array",
                SchemaFieldType::Object => "object",
                _ => "object",
            };

            // Set the 'type' field in the property object
            Reflect::set(&prop_obj, &JsValue::from_str("type"), &JsValue::from_str(prop_type_str))?;

            // If you have other fields like 'maxLength', 'minLength', etc., set them here as well
            // Example:
            if let Some(max_length) = property.max_length {
                Reflect::set(&prop_obj, &JsValue::from_str("maxLength"), &JsValue::from_f64(max_length as f64))?;
            }

            // Set the property object in the result object under the property name
            Reflect::set(&result, &JsValue::from_str(key), &prop_obj)?;
        }

        // Return the result as a JsValue
        Ok(result.into())
    }

}


#[cfg(feature = "browser")]
use wasm_bindgen_test::{wasm_bindgen_test_configure};
use crate::schema::property_type::{SchemaFieldType};

#[cfg(feature = "browser")]
wasm_bindgen_test_configure!(run_in_browser);



#[wasm_bindgen_test]
fn test_schema_creation() {
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"},
            "age": {"type": "number"}
        }
    }"#;
    let schema_value = JSON::parse(&schema_js).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    assert_eq!(schema.get_version(), 1);
    assert_eq!(schema.get_primary_key(), "id");
    assert_eq!(schema.get_schema_type(), "object");
}

#[wasm_bindgen_test]
fn test_schema_validation() {
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": {"type": "string"}
        }
    }"#;
    let schema_value = JSON::parse(schema_js).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    assert!(schema.is_valid().is_ok());
}


#[wasm_bindgen_test]
fn test_invalid_schema() {
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "invalid",
        "properties": {
            "id": {"type": "string"}
        }
    }"#;
    let schema_value = JSON::parse(schema_js).unwrap();
    let result = Schema::create(schema_value);

    assert!(result.is_err());
}

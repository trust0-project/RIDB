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
use crate::schema::property::{Property, Required};

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
      * The names of the required top-level properties. Follows JSON Schema
      * semantics: only the listed properties are required.
      */
     required?: string[];
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
        [K in keyof T['properties']]: T['properties'][K];
    };

    /**
     * The names of the required top-level properties.
     */
    readonly required?: (Extract<keyof T['properties'], string>)[];
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
    pub(crate) encrypted: Option<Vec<String>>,
    /// The names of the required top-level properties (JSON Schema semantics:
    /// only listed properties are required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>
}


#[wasm_bindgen]
impl Schema {

    #[wasm_bindgen(js_name="validate")]
    pub fn validate_document(&self, document: JsValue) -> Result<(), RIDBError> {
        let encrypted = self.encrypted.clone().unwrap_or_default();
        self.validate_object(&document, &self.properties, self.required.as_ref(), &encrypted)
    }

    /// Validates a JS object value against a set of properties.
    ///
    /// Requiredness precedence for each property (see [`Required`]):
    ///  1. a property-level boolean flag wins (`false` -> optional, `true` -> required);
    ///  2. otherwise, the property is required iff it is listed in the container's
    ///     `required` array (JSON Schema semantics);
    ///  3. otherwise (no array, no flag) the property is optional. An omitted `required`
    ///     array is therefore equivalent to `[]`, matching JSON Schema (and the
    ///     compile-time `CreateDoc`/`IsCreateOptional` types).
    ///
    /// Encrypted fields are exempt from the presence check (they are populated later in
    /// the pipeline). Nested object properties are validated recursively against their
    /// own `properties` and (array-form) `required`.
    fn validate_object(
        &self,
        document: &JsValue,
        properties: &HashMap<String, Property>,
        container_required: Option<&Vec<String>>,
        encrypted: &[String],
    ) -> Result<(), RIDBError> {
        for (key, prop) in properties {
            let value = Reflect::get(document, &JsValue::from_str(key))
                .map_err(|_e| {
                    JsValue::from(
                        RIDBError::validation(
                            &format!("Missing required property '{}'", key),
                            14
                        )
                    )
                })?;

            // Determine whether this property is required, applying the precedence above.
            let is_required = match &prop.required {
                Some(Required::Flag(flag)) => *flag,
                _ => container_required.is_some_and(|arr| arr.contains(key)),
            };

            // Only an absent (`undefined`) value counts as "missing". A `null` value is
            // present and must be validated against the declared type, so it falls through
            // to `is_type_correct` below (which rejects `null` for `string`, `object`,
            // etc.). This prevents optional fields from silently persisting `null`.
            if value.is_undefined() {
                if is_required && !encrypted.contains(key) {
                    return Err(
                        RIDBError::validation(
                            &format!("Missing required property '{}'", key),
                            15
                        )
                    );
                }
            } else {
                if !self.is_type_correct(key, &value, prop)? {
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

                // Recurse into nested object properties. Only the array form of
                // `required` names nested children; a boolean flag does not.
                if prop.property_type == SchemaFieldType::Object {
                    if let Some(nested_props) = &prop.properties {
                        let nested_required = match &prop.required {
                            Some(Required::Fields(fields)) => Some(fields),
                            _ => None,
                        };
                        self.validate_object(&value, nested_props, nested_required, &[])?;
                    }
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

        // Every name listed in `required` must be a defined property.
        if let Some(required) = &self.required {
            for name in required {
                if !self.properties.contains_key(name) {
                    return Err(
                        RIDBError::validation(
                            &format!("Required property '{}' is not defined in properties", name),
                            30
                        )
                    );
                }
            }
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

    /// Retrieves the required top-level property names, if any.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - The required property names, if any.
    #[wasm_bindgen(getter, js_name="required")]
    pub fn get_required(&self) -> Option<Vec<String>> {
        self.required.clone()
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

#[wasm_bindgen_test]
fn test_schema_required_subset_of_properties() {
    // `required` naming a property that is not defined must fail validation.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "missing"],
        "properties": {
            "id": {"type": "string"}
        }
    }"#;
    let schema_value = JSON::parse(schema_js).unwrap();
    let result = Schema::create(schema_value);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_validate_document_required_present() {
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "name"],
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"},
            "nickname": {"type": "string"}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    // Required fields present, optional `nickname` omitted -> valid.
    let doc = r#"{ "id": "1", "name": "Alice" }"#;
    assert!(schema.validate_document(JSON::parse(doc).unwrap()).is_ok());
}

#[wasm_bindgen_test]
fn test_validate_document_required_missing() {
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "name"],
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    // Required `name` missing -> error.
    let doc = r#"{ "id": "1" }"#;
    assert!(schema.validate_document(JSON::parse(doc).unwrap()).is_err());
}

#[wasm_bindgen_test]
fn test_validate_document_no_array_all_optional() {
    // JSON Schema semantics: with no `required` array and no per-property flags,
    // every property is optional. An omitted `required` array is equivalent to `[]`,
    // so a document missing defined properties still validates. This matches the
    // compile-time `CreateDoc`/`IsCreateOptional` types.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1" }"#).unwrap()).is_ok());
    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1", "name": "A" }"#).unwrap()).is_ok());
    // A present property must still type-check.
    assert!(schema.validate_document(JSON::parse(r#"{ "id": 1 }"#).unwrap()).is_err());
}

#[wasm_bindgen_test]
fn test_validate_document_omitted_nested_required_equals_empty() {
    // Bug 1 regression: an object property with no nested `required` array must behave
    // identically to an explicit empty `required: []` (nothing required), so a partial
    // nested object validates instead of demanding every nested key.
    let omitted = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "profile"],
        "properties": {
            "id": {"type": "string"},
            "profile": {
                "type": "object",
                "properties": {
                    "email": {"type": "string"},
                    "bio": {"type": "string"}
                }
            }
        }
    }"#;
    let empty = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "profile"],
        "properties": {
            "id": {"type": "string"},
            "profile": {
                "type": "object",
                "required": [],
                "properties": {
                    "email": {"type": "string"},
                    "bio": {"type": "string"}
                }
            }
        }
    }"#;
    let doc = r#"{ "id": "1", "profile": { "bio": "hi" } }"#;

    let schema_omitted = Schema::create(JSON::parse(omitted).unwrap()).unwrap();
    let schema_empty = Schema::create(JSON::parse(empty).unwrap()).unwrap();

    assert!(schema_omitted.validate_document(JSON::parse(doc).unwrap()).is_ok());
    assert!(schema_empty.validate_document(JSON::parse(doc).unwrap()).is_ok());
}

#[wasm_bindgen_test]
fn test_validate_document_legacy_required_false() {
    // Legacy per-property `required: false` keeps a field optional with no array present.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string", "required": false}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1" }"#).unwrap()).is_ok());
}

#[wasm_bindgen_test]
fn test_validate_document_flag_overrides_array() {
    // A property-level boolean flag overrides the container `required` array:
    // `name` is listed as required but flagged optional; `nickname` is unlisted but
    // flagged required.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "name"],
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string", "required": false},
            "nickname": {"type": "string", "required": true}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    // `name` omitted (flag false wins over array) but `nickname` present -> ok.
    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1", "nickname": "nn" }"#).unwrap()).is_ok());
    // `nickname` omitted (flag true) -> error even though it is not in the array.
    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1", "name": "A" }"#).unwrap()).is_err());
}

#[wasm_bindgen_test]
fn test_validate_document_nested_required() {
    // Nested object properties are validated against their own `required` list.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id", "profile"],
        "properties": {
            "id": {"type": "string"},
            "profile": {
                "type": "object",
                "required": ["email"],
                "properties": {
                    "email": {"type": "string"},
                    "bio": {"type": "string"}
                }
            }
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    let valid = r#"{ "id": "1", "profile": { "email": "a@b.c" } }"#;
    assert!(schema.validate_document(JSON::parse(valid).unwrap()).is_ok());

    // Nested required `email` missing -> error.
    let invalid = r#"{ "id": "1", "profile": { "bio": "hi" } }"#;
    assert!(schema.validate_document(JSON::parse(invalid).unwrap()).is_err());
}

#[wasm_bindgen_test]
fn test_validate_document_null_is_type_checked() {
    // Bug 2 regression: `null` is a present value, not a missing one. It must be
    // validated against the declared type and fail for non-nullable types, even for
    // optional properties. Otherwise an optional field could silently persist `null`.
    let schema_js = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "required": ["id"],
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"}
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_js).unwrap()).unwrap();

    // Optional `name` explicitly set to null -> type error (not silently accepted).
    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1", "name": null }"#).unwrap()).is_err());
    // Required `id` set to null -> still an error.
    assert!(schema.validate_document(JSON::parse(r#"{ "id": null }"#).unwrap()).is_err());
    // Omitting the optional `name` entirely remains valid.
    assert!(schema.validate_document(JSON::parse(r#"{ "id": "1" }"#).unwrap()).is_ok());
}

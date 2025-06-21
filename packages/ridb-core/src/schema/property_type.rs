use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use serde::ser::Error as SerError;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export const SchemaFieldType = {
  /**
   * String type for text data
   */
  string: 'string' as const,
  
  /**
   * Number type for numeric data (integers and floats)
   */
  number: 'number' as const,
  
  /**
   * Boolean type for true/false values
   */
  boolean: 'boolean' as const,
  
  /**
   * Array type for ordered collections of items
   */
  array: 'array' as const,
  
  /**
   * Object type for nested document structures
   */
  object: 'object' as const,
};
"#;


#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SchemaFieldType {
    String="string",
    Number="number",
    Boolean="boolean",
    Array="array",
    Object="object",
}


impl Serialize for SchemaFieldType {
    /// Serializes a `SchemaFieldType` into a string value.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer to use for converting the `SchemaFieldType`.
    ///
    /// # Returns
    ///
    /// * `Result<S::Ok, S::Error>` - A result indicating success or failure.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        match self {
            SchemaFieldType::String => serializer.serialize_str("string"),
            SchemaFieldType::Number => serializer.serialize_str("number"),
            SchemaFieldType::Boolean => serializer.serialize_str("boolean"),
            SchemaFieldType::Array => serializer.serialize_str("array"),
            SchemaFieldType::Object => serializer.serialize_str("object"),
            _ => unreachable!("This variant should never be constructed"),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFieldType {
    /// Deserializes a string value into a `SchemaFieldType`.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The deserializer to use for converting the string value.
    ///
    /// # Returns
    ///
    /// * `Result<Self, D::Error>` - A result indicating success or failure.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PropertyTypeVisitor)
    }
}

/// Visitor for deserializing a `SchemaFieldType` from a string.
struct PropertyTypeVisitor;

impl<'de> Visitor<'de> for PropertyTypeVisitor {
    type Value = SchemaFieldType;

    /// Describes what the visitor expects to receive.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to use for displaying the expected value.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - A result indicating success or failure.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an PropertyType (String, Number, Boolean, Object or Array)")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match  value {
            0 =>  Ok(SchemaFieldType::String),
            1 => Ok(SchemaFieldType::Number),
            2 => Ok(SchemaFieldType::Boolean),
            3 => Ok(SchemaFieldType::Array),
            4 => Ok(SchemaFieldType::Object),
            _ => Err(E::invalid_value(de::Unexpected::Str("Wrong key"), &self)),
        }
    }

    /// Visits a string value and attempts to convert it into a `SchemaFieldType`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert.
    ///
    /// # Returns
    ///
    /// * `Result<SchemaFieldType, E>` - A result indicating success or failure.
    fn visit_str<E>(self, value: &str) -> Result<SchemaFieldType, E>
        where
            E: de::Error,
    {
        match value {
            "string" => Ok(SchemaFieldType::String),
            "number" => Ok(SchemaFieldType::Number),
            "boolean" => Ok(SchemaFieldType::Boolean),
            "array" => Ok(SchemaFieldType::Array),
            "object" => Ok(SchemaFieldType::Object),
            _ => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }
}

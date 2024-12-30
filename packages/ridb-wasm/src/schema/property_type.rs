use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use serde::ser::Error as SerError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropertyType {
    String="string",
    Number="number",
    Boolean="boolean",
    Array="array",
    Object="object",
}


impl Serialize for PropertyType {
    /// Serializes a `PropertyType` into a string value.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer to use for converting the `PropertyType`.
    ///
    /// # Returns
    ///
    /// * `Result<S::Ok, S::Error>` - A result indicating success or failure.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        match self {
            PropertyType::String => serializer.serialize_str("string"),
            PropertyType::Number => serializer.serialize_str("number"),
            PropertyType::Boolean => serializer.serialize_str("boolean"),
            PropertyType::Array => serializer.serialize_str("array"),
            PropertyType::Object => serializer.serialize_str("object"),
            _ => Err(SerError::custom("Wrong key")),
        }
    }
}

impl<'de> Deserialize<'de> for PropertyType {
    /// Deserializes a string value into a `PropertyType`.
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

/// Visitor for deserializing a `PropertyType` from a string.
struct PropertyTypeVisitor;

impl<'de> Visitor<'de> for PropertyTypeVisitor {
    type Value = PropertyType;

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
            0 =>  Ok(PropertyType::String),
            1 => Ok(PropertyType::Number),
            2 => Ok(PropertyType::Boolean),
            3 => Ok(PropertyType::Array),
            4 => Ok(PropertyType::Object),
            _ => Err(E::invalid_value(de::Unexpected::Str("Wrong key"), &self)),
        }
    }

    /// Visits a string value and attempts to convert it into a `PropertyType`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert.
    ///
    /// # Returns
    ///
    /// * `Result<PropertyType, E>` - A result indicating success or failure.
    fn visit_str<E>(self, value: &str) -> Result<PropertyType, E>
        where
            E: de::Error,
    {
        match value {
            "string" => Ok(PropertyType::String),
            "number" => Ok(PropertyType::Number),
            "boolean" => Ok(PropertyType::Boolean),
            "array" => Ok(PropertyType::Array),
            "object" => Ok(PropertyType::Object),
            _ => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }
}

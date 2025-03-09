pub mod options;

use std::collections::HashMap;
use js_sys::{Array,  Object, JSON};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::wasm_bindgen_test;
use crate::schema::Schema;
use js_sys::Reflect;
use crate::error::RIDBError;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Operators<T> = {
    $gte?: number,
    $gt?: number
    $lt?: number,
    $lte?: number,
    $eq?: T,
    $ne?: T
};

export type InOperator<T> = {  $in?: T[] };
export type NInOperator<T> = {  $nin?: T[] };

export type OperatorOrType<T> = T extends number ? 
    T | Operators<T> | InOperator<T> | NInOperator<T> : 
    T | InOperator<T> | NInOperator<T>;

export type LogicalOperators<T extends SchemaType> = {
    $and?: Partial<QueryType<T>>[];
    $or?: Partial<QueryType<T>>[];
};

export type QueryType<T extends SchemaType> = Partial<{
    [K in keyof T['properties']]: OperatorOrType<
        ExtractType<
            T['properties'][K]['type']
        >
    >
}> & LogicalOperators<T> | LogicalOperators<T>[];

export class Query<T extends SchemaType> {
    constructor(query: QueryType<T>, schema:Schema<T>);
    readonly query: QueryType<T>;
}
"#;


#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct Query {
    pub(crate) query: JsValue,
    pub(crate) schema: Schema
}

#[wasm_bindgen]
impl Query {
    #[wasm_bindgen(constructor)]
    pub fn new(query: JsValue, schema: Schema) -> Result<Query, RIDBError> {
        Ok(Query { query, schema })
    }

    #[wasm_bindgen(getter, js_name="query")]
    pub fn get_query(&self) -> Result<JsValue, RIDBError> {
        // Normalize the query
        let normalized_query = self.normalize_query(&self.query)?;
        Ok(normalized_query)
    }

    /// Returns the schema properties (fields) that are used in the query.
    /// The query may contain operators like $and, $or, $gt, $lt, etc.
    pub fn get_properties(&self) -> Result<Vec<String>, RIDBError> {
        let mut properties = Vec::new();

        fn collect_properties(value: &JsValue, props: &mut Vec<String>) -> Result<(), JsValue> {
            if value.is_array() {
                // Process array elements
                let arr = Array::from(value);
                for i in 0..arr.length() {
                    let elem = arr.get(i);
                    collect_properties(&elem, props)?;
                }
            } else if value.is_object() {
                // Process object properties
                let obj = Object::from(value.clone());
                let keys = Object::keys(&obj);
                for i in 0..keys.length() {
                    let key = keys.get(i).as_string().unwrap_or_default();
                    let val = Reflect::get(&obj, &JsValue::from(&key))?;

                    if key.starts_with('$') {
                        // Operator: process value recursively
                        collect_properties(&val, props)?;
                    } else {
                        // Property name: add if not already present
                        if !props.contains(&key) {
                            props.push(key.clone());
                        }
                        // Process value recursively
                        collect_properties(&val, props)?;
                    }
                }
            }
            // Non-object and non-array values are ignored
            Ok(())
        }

        collect_properties(&self.get_query()?, &mut properties)?;
        Ok(properties)
    }

    fn normalize_query(&self, query: &JsValue) -> Result<JsValue, RIDBError> {
        // 1) If it's an array, normalize each item and wrap them into an $and
        if query.is_array() {
            let arr = Array::from(query);
            let conditions = Array::new();
            for i in 0..arr.length() {
                let item = arr.get(i);
                let normalized_item = self.normalize_query(&item)?;
                conditions.push(&normalized_item);
            }
            // If there is exactly 1 element, just return that element
            if conditions.length() == 1 {
                Ok(conditions.get(0))
            } else {
                // Otherwise wrap them in $and
                let result = Object::new();
                Reflect::set(&result, &JsValue::from_str("$and"), &conditions)?;
                Ok(result.into())
            }
        }
        // 2) Otherwise, ensure it is an object
        else if query.is_object() {
            let obj = Object::from(query.clone());
            let keys = Object::keys(&obj);
            let conditions = Array::new();

            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let value = Reflect::get(query, &JsValue::from_str(&key))?;

                if key == "$and" || key == "$or" {
                    // Process the logical operator array
                    if !Array::is_array(&value) {
                        return Err(
                                RIDBError::from(format!("{} must be an array", key))
                    
                        );
                    }
                    let arr = Array::from(&value);
                    let processed_arr = Array::new();
                    for j in 0..arr.length() {
                        let item = arr.get(j);
                        let normalized_item = self.normalize_query(&item)?;
                        processed_arr.push(&normalized_item);
                    }

                    let operator_condition = Object::new();
                    Reflect::set(&operator_condition, &JsValue::from_str(&key), &processed_arr)?;
                    conditions.push(&operator_condition);
                } else {
                    // Normal field condition
                    let condition = Object::new();
                    Reflect::set(&condition, &JsValue::from_str(&key), &value)?;
                    conditions.push(&condition);
                }
            }

            // Wrap conditions into $and if there are multiple
            if conditions.length() == 1 {
                Ok(conditions.get(0))
            } else {
                let result = Object::new();
                Reflect::set(&result, &JsValue::from_str("$and"), &conditions)?;
                Ok(result.into())
            }
        } else {
            // If it's neither an array nor an object, reject
            Err(
                    RIDBError::from("Query must be an object or an array at the top level")
                
            )
        }
    }

    pub fn parse(&self) -> Result<JsValue, RIDBError> {
        self.process_query(&self.query)
    }

     fn extract_schema_properties(&self, properties_jsvalue: &JsValue) -> Result<HashMap<String, String>, RIDBError> {
        if !properties_jsvalue.is_object() {
            return Err(
                    RIDBError::validation("Properties is not an object", 0)
            );
        }
        let mut properties = HashMap::new();
        let keys = Object::keys(&Object::from(properties_jsvalue.clone()));

        for key in keys {
            let value = Reflect::get(properties_jsvalue, &key)?;
            if !value.is_object() {
                return Err(
                        RIDBError::validation(
                            format!("Property '{}' is not an object", key.as_string().unwrap()).as_str(),
                            0
                        )
                );
            }
            let prop_type = Reflect::get(&value, &JsValue::from_str("type"))?;
            if prop_type.is_string() {
                properties.insert(key.as_string().unwrap(), prop_type.as_string().unwrap());
            } else {
                return Err(
                        RIDBError::validation(
                            format!("Property '{}' does not have a 'type' field", key.as_string().unwrap()).as_str(),
                            0
                        )
                );
            }
        }
        Ok(properties)
    }

    pub fn process_query(&self, query: &JsValue) -> Result<JsValue, RIDBError> {
        let properties_jsvalue = self.schema.get_properties()?;
        let properties = self.extract_schema_properties(&properties_jsvalue)?;
        if !query.is_object() {
            return Err(
                    RIDBError::validation("Query must be an object", 0)
            );
        }
        let result = Object::new();
        let keys = Object::keys(&Object::from(query.clone()));
        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(query, &JsValue::from_str(&key))?;
            if key == "$and" || key == "$or" {
                if !Array::is_array(&value) {
                    return Err(
                            RIDBError::validation(
                                format!("{} must be an array", key).as_str(),
                                0
                            )
                    );
                }
                let arr = Array::from(&value);
                let processed_arr = Array::new();
                for j in 0..arr.length() {
                    let item = arr.get(j);
                    let processed_item = self.process_query(&item)?;
                    processed_arr.push(&processed_item);
                }
                Reflect::set(&result, &JsValue::from_str(&key), &processed_arr)?;
            } else {
                if let Some(property_type) = properties.get(&key) {
                    let processed_value = self.process_value(&value, property_type)?;
                    Reflect::set(&result, &JsValue::from_str(&key), &processed_value)?;
                } else if properties.get("id").is_some() && self.schema.clone().indexes.is_some() {
                    if let Some(property_type) = properties.get("id") {
                        let processed_value = self.process_value(&value, property_type)?;
                        Reflect::set(&result, &JsValue::from_str(&key), &processed_value)?;
                    } else {
                        return Err(
                                RIDBError::validation(
                                    format!("Invalid property: {} does not exist", key).as_str(),
                                    0
                                )
                        );
                    }
                } else {
                    return Err(
                            RIDBError::validation(
                                format!("Invalid property: {} does not exist", key).as_str(),
                                0
                            )
                    );
                }
            }
        }
        Ok(
            JsValue::from(
                result
            )
        )
    }

    fn process_value(&self, value: &JsValue, property_type: &str) -> Result<JsValue, RIDBError> {
        if value.is_object() && !Array::is_array(value) {
            // Value is an object, process operators
            let result = Object::new();
            let keys = Object::keys(&Object::from(value.clone()));
            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let val = Reflect::get(value, &JsValue::from_str(&key))?;
                if ["$gte", "$gt", "$lt", "$lte", "$in", "$nin", "$eq", "$ne"].contains(&key.as_str()) {
                    // Validate operator value
                    self.validate_operator_value(&key, &val, property_type)?;
                    Reflect::set(&result, &JsValue::from_str(&key), &val)?;
                } else {
                    return Err(
                            RIDBError::validation(
                                format!("Invalid operator: {}", key).as_str(),
                                0
                            )
                    );
                }
            }
            Ok(result.into())
        } else {
            // Direct value, check that it matches the property type
            self.validate_value(value, property_type)?;
            Ok(value.clone())
        }
    }

    fn validate_operator_value(&self, operator: &str, value: &JsValue, property_type: &str) -> Result<(), RIDBError> {
        match operator {
            "$eq" => {
                self.validate_value(value, property_type)?;
                Ok(())
            }
            "$ne" => {
                self.validate_value(value, property_type)?;
                Ok(())
            }
            "$in" => {
                if !Array::is_array(value) {
                    return Err(
                            RIDBError::validation(
                                format!("{} operator requires an array", operator).as_str(),
                                0
                            )
                    );
                }
                let arr = Array::from(value);
                for i in 0..arr.length() {
                    let item = arr.get(i);
                    self.validate_value(&item, property_type)?;
                }
                Ok(())
            }
            "$nin" => {
                if !Array::is_array(value) {
                    return Err(
                            RIDBError::validation(
                                format!("{} operator requires an array", operator).as_str(),
                                0
                            )
                    );
                }
                let arr = Array::from(value);
                for i in 0..arr.length() {
                    let item = arr.get(i);
                    self.validate_value(&item, property_type)?;
                }
                Ok(())
            }
            "$gte" | "$gt" | "$lt" | "$lte" => {
                self.validate_value(value, property_type)
            }
            _ => {
                Err(
                        RIDBError::validation(
                            format!("Unsupported operator: {}", operator).as_str(),
                            0
                        )
                )
            },
        }
    }

    fn validate_value(&self, value: &JsValue, property_type: &str) -> Result<(), RIDBError> {
        match property_type {
            "number" => {
                if value.as_f64().is_some() {
                    Ok(())
                } else {
                    Err(
                            RIDBError::validation("Expected a number", 0)
                    )
                }   
            }
            "string" => {
                if value.is_string() {
                    Ok(())
                } else {
                    Err(
                            RIDBError::validation("Expected a string", 0)
                    )
                }
            }
            "boolean" => {
                if value.is_truthy() || value.is_falsy() {
                    Ok(())
                } else {
                    Err(
                            RIDBError::validation("Expected a boolean", 0)
                    )
                }
            }
            _ => {
                Err(
                        RIDBError::validation(
                            format!("Unsupported property type: {}", property_type).as_str(),
                            0
                        )
                )
            },
        }
    }

    /// Returns the value of a property from the (normalized) query by its name.
    /// This will scan the normalized query structure (including arrays, $and/$or blocks, etc.)
    /// to find the first occurrence of the given property name and return its corresponding value.
    ///
    /// If not found, an error is returned.
    ///
    /// Example:
    ///   let val = query.get("age")?;
    ///   // val is a JsValue that might be a number, string, boolean, array, or object (e.g., { "$gt": 30 })
    #[wasm_bindgen(js_name = "get")]
    pub fn get(&self, property_name: &str) -> Result<JsValue, RIDBError> {
        let normalized = self.get_query()?;
        match Self::find_property_value(&normalized, property_name) {
            Some(val) => Ok(val),
            None => Err(
                    RIDBError::validation(
                        format!(
                            "Property '{}' not found in query",
                            property_name
                        ).as_str(),
                        0
                    )
            ),
        }
    }
}

impl Query {
    /// Recursively searches the (already normalized) query structure for the first occurrence
    /// of a given property name. Returns Some(JsValue) if found, otherwise None.
    fn find_property_value(value: &JsValue, property_name: &str) -> Option<JsValue> {
        if value.is_array() {
            // For arrays (e.g. $and/$or arrays), check each element
            let arr = Array::from(value);
            for i in 0..arr.length() {
                let elem = arr.get(i);
                if let Some(val) = Self::find_property_value(&elem, property_name) {
                    return Some(val);
                }
            }
        } else if value.is_object() {
            // For objects, check each key
            let obj = Object::from(value.clone());
            let keys = Object::keys(&obj);

            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let val = Reflect::get(&obj, &JsValue::from_str(&key)).ok()?;

                // Our property name matches this key
                if key == property_name {
                    // Return the matched value immediately.
                    return Some(val);
                }

                // If it's an operator or a nested object, recurse
                if key.starts_with('$') || val.is_object() || val.is_array() {
                    if let Some(v) = Self::find_property_value(&val, property_name) {
                        return Some(v);
                    }
                }
            }
        }
        // If it's neither an array nor an object (or we didn't find anything), return None
        None
    }
}

#[wasm_bindgen_test]
fn test_get_properties_simple_fields() {
    let schema_str = r#"{
        "version": 1,
        "primaryKey":"id",
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" },
            "name": { "type": "string" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test with simple fields without any operators
    let query_json = r#"
    {
        "name": "John Doe",
        "age": 30
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["name", "age"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_with_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "price": { "type": "number" },
            "stock": { "type": "number" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test with operators like $gt, $lt
    let query_json = r#"
    {
        "price": { "$gt": 100 },
        "stock": { "$lt": 50 }
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["price", "stock"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_with_logical_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "category": { "type": "string" },
            "available": { "type": "boolean" },
            "brand": { "type": "string" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test with logical operators like $and, $or
    let query_json = r#"
    {
        "$and": [
            { "category": "electronics" },
            { "available": true }
        ],
        "$or": [
            { "brand": "BrandA" },
            { "brand": "BrandB" }
        ]
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["category", "available", "brand"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_nested_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" },
            "status": { "type": "string" },
            "membership": { "type": "string" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test with nested operators
    let query_json = r#"
    {
        "status": "active",
        "details": {
            "$and": [
                { "age": { "$gt": 18 } },
                { "membership": { "$ne": "basic" } }
            ]
        }
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["status", "details", "age", "membership"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_array_values() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "tags": { "type": "string" },
            "ratings": { "type": "number" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test properties with array values
    let query_json = r#"
    {
        "tags": ["new", "sale", "popular"],
        "ratings": { "$in": [4, 5] }
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["tags", "ratings"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_empty_query() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
             "id": { "type": "string" },
            "tags": { "type": "string" },
            "ratings": { "type": "number" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    // Test with an empty query
    let query_json = "{}";
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let expected_props: Vec<String> = Vec::new();
    assert_eq!(props, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_deeply_nested() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "level1": {
                "type": "object",
                "properties": {
                    "level2": {
                        "type":"object",
                        "properties": {
                            "level3": {
                                "type":"object",
                                "properties": {
                                    "field1": { "type": "string"},
                                    "field2": { "type": "string"}
                                }
                            }
                        }
                    }
                }
            }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test with deeply nested query
    let query_json = r#"
    {
        "level1": {
            "level2": {
                "level3": {
                    "$and": [
                        { "field1": { "$eq": "value1" } },
                        { "field2": { "$ne": "value2" } }
                    ]
                }
            }
        }
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();
    let mut expected_props = vec!["level1", "level2", "level3", "field1", "field2"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_with_multiple_same_props() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test query where the same property appears multiple times
    let query_json = r#"
    {
        "name": "Alice",
        "$or": [
            { "name": "Bob" },
            { "name": "Charlie" }
        ]
    }
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let expected_props = vec!["name"];

    assert_eq!(props, expected_props);
}

#[wasm_bindgen_test]
fn test_get_properties_with_array_at_top_level() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" },
            "age": { "type": "number" }
        }
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();
    // Test query where the top-level query is an array
    let query_json = r#"
    [
        { "name": "Alice" },
        { "name": "Bob" },
        { "age": { "$gt": 25 } }
    ]
    "#;
    let query_js = JSON::parse(query_json).unwrap();
    let query = Query::new(query_js, schema).unwrap();
    let props = query.get_properties().unwrap();

    let mut expected_props = vec!["name", "age"];
    expected_props.sort();
    let mut props_sorted = props.clone();
    props_sorted.sort();

    assert_eq!(props_sorted, expected_props);
}

#[wasm_bindgen_test]
fn test_query_parse_operator_wrong_type() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$gt": "thirty" }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err());

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Expected a number");
}

#[wasm_bindgen_test]
fn test_query_parse_in_operator() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "status": { "$in": ["active", "pending"] }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_in_operator_wrong_type() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$in": ["thirty", "forty"] }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err());

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Expected a number");
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_simple_attributes() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" }
        }
    }"#;
    let query_str = r#"{
        "id": "123",
        "name": "Alice"
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_value, schema).unwrap();

    let normalized_query = query.get_query().unwrap();
    let expected_str = r#"{
        "$and": [
            { "id": "123" },
            { "name": "Alice" }
        ]
    }"#;
    let expected_value = JSON::parse(expected_str).unwrap();

    assert_eq!(
        JSON::stringify(&normalized_query).unwrap(),
        JSON::stringify(&expected_value).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_with_logical_operator() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }"#;
    let query_str = r#"{
        "id": "123",
        "$or": [
            { "age": { "$gt": 30 } },
            { "age": { "$lt": 20 } }
        ]
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_value, schema).unwrap();

    let normalized_query = query.get_query().unwrap();
    let expected_str = r#"{
        "$and": [
            { "id": "123" },
            {
                "$or": [
                    { "age": { "$gt": 30 } },
                    { "age": { "$lt": 20 } }
                ]
            }
        ]
    }"#;
    let expected_value = JSON::parse(expected_str).unwrap();

    assert_eq!(
        JSON::stringify(&normalized_query).unwrap(),
        JSON::stringify(&expected_value).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_nested_logical_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" },
            "age": { "type": "number" },
            "role": { "type": "string" }
        }
    }"#;
    let query_str = r#"{
        "$or": [
            {
                "$and": [
                    { "status": "active" },
                    { "age": { "$gte": 30 } }
                ]
            },
            { "role": "admin" }
        ]
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_value, schema).unwrap();

    let normalized_query = query.get_query().unwrap();
    let expected_str = r#"{
        "$or": [
            {
                "$and": [
                    { "status": "active" },
                    { "age": { "$gte": 30 } }
                ]
            },
            { "role": "admin" }
        ]
    }"#;
    let expected_value = JSON::parse(expected_str).unwrap();

    assert_eq!(
        JSON::stringify(&normalized_query).unwrap(),
        JSON::stringify(&expected_value).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_only_logical_operator() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" },
            "score": { "type": "number" }
        }
    }"#;
    let query_str = r#"{
        "$and": [
            { "age": { "$gt": 18 } },
            { "score": { "$lte": 100 } }
        ]
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_value, schema).unwrap();

    let normalized_query = query.get_query().unwrap();
    let expected_str = r#"{
        "$and": [
            { "age": { "$gt": 18 } },
            { "score": { "$lte": 100 } }
        ]
    }"#;
    let expected_value = JSON::parse(expected_str).unwrap();

    assert_eq!(
        JSON::stringify(&normalized_query).unwrap(),
        JSON::stringify(&expected_value).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_complex_mixed() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" },
            "age": { "type": "number" },
            "city": { "type": "string" },
            "status": { "type": "string" }
        }
    }"#;
    let query_str = r#"{
        "name": "Bob",
        "$or": [
            { "city": "New York" },
            {
                "$and": [
                    { "age": { "$lt": 30 } },
                    { "status": "active" }
                ]
            }
        ]
    }"#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_value, schema).unwrap();

    let normalized_query = query.get_query().unwrap();
    let expected_str = r#"{
        "$and": [
            { "name": "Bob" },
            {
                "$or": [
                    { "city": "New York" },
                    {
                        "$and": [
                            { "age": { "$lt": 30 } },
                            { "status": "active" }
                        ]
                    }
                ]
            }
        ]
    }"#;
    let expected_value = JSON::parse(expected_str).unwrap();

    assert_eq!(
        JSON::stringify(&normalized_query).unwrap(),
        JSON::stringify(&expected_value).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_query_parse_empty_query() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" }
        }
    }"#;
    let query_str = "{}";
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_age_query() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }"#;
    let query_str = r#"{
        "age":25
    }"#;
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_non_object_query() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" }
        }
    }"#;
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JsValue::from_str("not an object");
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err());

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Query must be an object");
}

#[wasm_bindgen_test]
fn test_query_parse_multiple_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }"#;
    let query_str = r#"{
        "age": { "$gt": 20, "$lt": 30 }
    }"#;
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_invalid_in_operator() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" }
        }
    }"#;
    let query_str = r#"{
        "status": { "$in": "not-an-array" }
    }"#;
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err());

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: $in operator requires an array");
}

#[wasm_bindgen_test]
fn test_query_parse_empty_logical_operators() {
    let schema_str = r#"{
        "version": 1,
        "type": "object",
        "primaryKey":"id",
        "properties": {
            "id": { "type": "string" }
        }
    }"#;
    let query_str = r#"{
        "$and": []
    }"#;
    let schema = Schema::create(JSON::parse(schema_str).unwrap()).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_nin_operator() {
    // This test checks that the $nin operator works correctly for string-based properties
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "status": { "$nin": ["active", "inactive"] }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok(), "Parsing query with $nin should succeed for correct types.");
}

#[wasm_bindgen_test]
fn test_query_parse_nin_operator_wrong_type() {
    // This test checks that using $nin on a number property with string values fails
    let schema_str = r#"
    {
        "version": 1,
        "type": "object",
        "primaryKey": "id",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$nin": ["twenty", "thirty"] }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err(), "Parsing query with $nin operator on wrong value types should fail.");

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Expected a number");
}

#[wasm_bindgen_test]
fn test_query_parse_eq_operator() {
    // This test checks that the $eq operator works correctly for string-based properties
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "status": { "$eq": "active" }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok(), "Validation Error: Parsing query with $eq should succeed for correct types.");
}

#[wasm_bindgen_test]
fn test_query_parse_eq_operator_wrong_type() {
    // This test checks using the $eq operator on a number property with a non-number value
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$eq": "thirty" }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err(), "Parsing query with $eq operator on wrong value types should fail.");

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Expected a number");
}

#[wasm_bindgen_test]
fn test_query_parse_ne_operator() {
    // This test checks that the $ne operator works correctly for string-based properties
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "status": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "status": { "$ne": "inactive" }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_ok(), "Validation Error: Parsing query with $ne should succeed for correct types.");
}

#[wasm_bindgen_test]
fn test_query_parse_ne_operator_wrong_type() {
    // This test checks using the $ne operator on a number property with a non-number value
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$ne": "forty" }
    }
    "#;
    let schema_value = JSON::parse(schema_str).unwrap();
    let schema = Schema::create(schema_value).unwrap();

    let query_js_value = JSON::parse(query_str).unwrap();
    let query = Query::new(query_js_value, schema).unwrap();

    let result = query.parse();
    assert!(result.is_err(), "Validation Error: Parsing query with $ne operator on wrong value types should fail.");

    let error_js = result.err().unwrap();
    let ridb_err = RIDBError::from(error_js);
    assert_eq!(ridb_err.get_message(), "Validation Error: Expected a number");
}
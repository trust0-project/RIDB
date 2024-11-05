use std::collections::HashMap;
use js_sys::{Array,  Object, JSON};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::wasm_bindgen_test;
use crate::schema::Schema;
use js_sys::Reflect;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Operators = {
    $gte?: number,
    $gt?: number
    $lt?: number,
    $lte?: number
};
export type InOperator<T> = {  $in?: T[] };
export type OperatorOrType<T> = T extends number ? T | Operators | InOperator<T> : T | InOperator<T>;
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
    readonly query: QueryType<T>
}
"#;

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct Query {
    pub(crate) query: JsValue,
    pub(crate) schema: Schema,
}

#[wasm_bindgen]
impl Query {
    #[wasm_bindgen(constructor)]
    pub fn new(query: JsValue, schema: Schema) -> Result<Query, JsValue> {
        Ok(Query { query, schema })
    }

    #[wasm_bindgen(getter, js_name="query")]
    pub fn get_query(&self) -> Result<JsValue, JsValue> {
        // Normalize the query
        let normalized_query = self.normalize_query(&self.query)?;
        Ok(normalized_query)
    }

    fn normalize_query(&self, query: &JsValue) -> Result<JsValue, JsValue> {
        // Ensure query is an object
        if !query.is_object() {
            return Err(JsValue::from_str("Query must be an object"));
        }

        // Separate attributes and logical operators
        let obj = Object::from(query.clone());
        let keys = Object::keys(&obj);
        let conditions = Array::new();

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(query, &JsValue::from_str(&key))?;

            if key == "$and" || key == "$or" {
                // Process the logical operator recursively
                if !Array::is_array(&value) {
                    return Err(JsValue::from_str(&format!("{} must be an array", key)));
                }
                let arr = Array::from(&value);
                let processed_arr = Array::new();

                for i in 0..arr.length() {
                    let item = arr.get(i);
                    let normalized_item = self.normalize_query(&item)?;
                    processed_arr.push(&normalized_item);
                }

                let operator_condition = Object::new();
                Reflect::set(&operator_condition, &JsValue::from_str(&key), &processed_arr)?;

                // Add the operator condition to conditions
                conditions.push(&operator_condition);
            } else {
                // Attribute: wrap it into a condition object
                let condition = Object::new();
                Reflect::set(&condition, &JsValue::from_str(&key), &value)?;
                conditions.push(&condition);
            }
        }

        // Wrap conditions into $and if there are multiple conditions
        if conditions.length() == 1 {
            // Only one condition, return it directly
            Ok(conditions.get(0))
        } else {
            // Multiple conditions, wrap into $and
            let result = Object::new();
            Reflect::set(&result, &JsValue::from_str("$and"), &conditions)?;
            Ok(result.into())
        }
    }

    pub fn parse(&self) -> Result<JsValue, JsValue> {
        self.process_query(&self.query)
    }

    fn extract_properties(&self, properties_jsvalue: &JsValue) -> Result<HashMap<String, String>, JsValue> {
        if !properties_jsvalue.is_object() {
            return Err(JsValue::from_str("Properties is not an object"));
        }
        let mut properties = HashMap::new();
        let keys = Object::keys(&Object::from(properties_jsvalue.clone()));

        for key in keys {
            let value = Reflect::get(properties_jsvalue, &key)?;
            if !value.is_object() {
                return Err(JsValue::from_str(&format!("Property '{}' is not an object", key.as_string().unwrap())));
            }
            let prop_type = Reflect::get(&value, &JsValue::from_str("type"))?;
            if prop_type.is_string() {
                properties.insert(key.as_string().unwrap(), prop_type.as_string().unwrap());
            } else {
                return Err(JsValue::from_str(&format!("Property '{}' does not have a 'type' field", key.as_string().unwrap())));
            }
        }
        Ok(properties)
    }

    fn process_query(&self, query: &JsValue) -> Result<JsValue, JsValue> {
        // Get properties from the schema
        let properties_jsvalue = self.schema.get_properties()?;

        // Extract properties and their types from the properties_jsvalue
        let properties = self.extract_properties(&properties_jsvalue)?;

        if !query.is_object() {
            return Err(JsValue::from_str("Query must be an object"));
        }
        let result = Object::new();
        let keys = Object::keys(&Object::from(query.clone()));

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(query, &JsValue::from_str(&key))?;
            if key == "$and" || key == "$or" {
                // Handle logical operators
                if !Array::is_array(&value) {
                    return Err(JsValue::from_str(&format!("{} must be an array", key)));
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
                // Check if key is a valid property
                if let Some(property_type) = properties.get(&key) {
                    // Process the value
                    let processed_value = self.process_value(&value, property_type)?;
                    Reflect::set(&result, &JsValue::from_str(&key), &processed_value)?;
                } else {
                    return Err(JsValue::from_str(&format!("Invalid property: {}", key)));
                }
            }
        }
        Ok(result.into())
    }

    fn process_value(&self, value: &JsValue, property_type: &str) -> Result<JsValue, JsValue> {
        if value.is_object() && !Array::is_array(value) {
            // Value is an object, process operators
            let result = Object::new();
            let keys = Object::keys(&Object::from(value.clone()));
            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let val = Reflect::get(value, &JsValue::from_str(&key))?;
                if ["$gte", "$gt", "$lt", "$lte", "$in"].contains(&key.as_str()) {
                    // Validate operator value
                    self.validate_operator_value(&key, &val, property_type)?;
                    Reflect::set(&result, &JsValue::from_str(&key), &val)?;
                } else {
                    return Err(JsValue::from_str(&format!("Invalid operator: {}", key)));
                }
            }
            Ok(result.into())
        } else {
            // Direct value, check that it matches the property type
            self.validate_value(value, property_type)?;
            Ok(value.clone())
        }
    }

    fn validate_operator_value(&self, operator: &str, value: &JsValue, property_type: &str) -> Result<(), JsValue> {
        match operator {
            "$in" => {
                if !Array::is_array(value) {
                    return Err(JsValue::from_str(&format!("{} operator requires an array", operator)));
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
                Err(JsValue::from_str(&format!("Unsupported operator: {}", operator)))
            },
        }
    }

    fn validate_value(&self, value: &JsValue, property_type: &str) -> Result<(), JsValue> {
        match property_type {
            "number" => {
                if value.as_f64().is_some() {
                    Ok(())
                } else {
                    Err(JsValue::from_str("Expected a number"))
                }
            }
            "string" => {
                if value.is_string() {
                    Ok(())
                } else {
                    Err(JsValue::from_str("Expected a string"))
                }
            }
            "boolean" => {
                if value.is_truthy() || value.is_falsy() {
                    Ok(())
                } else {
                    Err(JsValue::from_str("Expected a boolean"))
                }
            }
            _ => {
                Err(JsValue::from_str(&format!("Unsupported property type: {}", property_type)))
            },
        }
    }
}


#[wasm_bindgen_test]
fn test_query_parse_valid() {
    let schema_str = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string", "maxLength": 60 },
            "name": { "type": "string" },
            "age": { "type": "number" }
        }
    }"#;
    let query_str = r#"{
        "id":"12345",
        "name": "John",
        "age": { "$gt": 30 }
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(
        schema_js_value
    ).expect("Could not create schema");
    let query_js_value = JSON::parse(query_str).expect("QueryValue");
    let query = Query::new(
        query_js_value, schema).expect("could not create q");
    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_invalid_property() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "id": { "type": "string", "maxLength": 60 },
            "name": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "nonexistent": "value"
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();

    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().as_string().unwrap(),
        "Invalid property: nonexistent"
    );
}

#[wasm_bindgen_test]
fn test_query_parse_invalid_type() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": "thirty"
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().as_string().unwrap(),
        "Expected a number"
    );
}

#[wasm_bindgen_test]
fn test_query_parse_logical_operators() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "$and": [
            { "name": "Alice" },
            { "age": { "$gte": 25 } }
        ]
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_query_parse_invalid_operator() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$unknown": 30 }
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().as_string().unwrap(),
        "Invalid operator: $unknown"
    );
}

#[wasm_bindgen_test]
fn test_query_parse_operator_wrong_type() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$gt": "thirty" }
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().as_string().unwrap(),
        "Expected a number"
    );
}

#[wasm_bindgen_test]
fn test_query_parse_in_operator() {
    let schema_str = r#"
    {
        "version": 1,
        "primaryKey": "id",
        "type": "object",
        "properties": {
            "status": { "type": "string" }
        }
    }
    "#;
    let query_str = r#"
    {
        "status": { "$in": ["active", "pending"] }
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
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
        "properties": {
            "age": { "type": "number" }
        }
    }
    "#;
    let query_str = r#"
    {
        "age": { "$in": ["thirty", "forty"] }
    }
    "#;
    let schema_value = JSON::parse(&schema_str).unwrap();
    let schema_js_value = schema_value;
    let schema = Schema::create(schema_js_value).unwrap();
    let query = Query::new(JSON::parse(query_str).unwrap(), schema).unwrap();
    let result = query.parse();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().as_string().unwrap(),
        "Expected a number"
    );
}

#[wasm_bindgen_test]
fn test_query_get_query_normalization_simple_attributes() {
    let schema_str = r#"{
        "version": 1,
        "primaryKey": "id",
        "type": "object",
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
        "primaryKey": "id",
        "type": "object",
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
        "primaryKey": "id",
        "type": "object",
        "properties": {
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
        "primaryKey": "id",
        "type": "object",
        "properties": {
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
        "primaryKey": "id",
        "type": "object",
        "properties": {
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

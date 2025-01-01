use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::query::Query;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CoreStorage {
    /**
    * @param {any} document
    * @param {Query} query
    * @returns {boolean}
    */
    matchesQuery(document: any, query: Query<any>): boolean;
}
"#;





#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct CoreStorage {

}

#[wasm_bindgen]
impl CoreStorage {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }


    #[wasm_bindgen(js_name = getPrimaryKey)]
    pub fn  get_primary_key(&self, value:JsValue) -> Result<String, JsValue> {
        if value.is_undefined() || value.is_null() {
            return Err(JsValue::from_str("Document must contain a primary key"));
        }
        if let Some(s) = value.as_string() {
            Ok(s)
        } else if let Some(n) = value.as_f64() {
            Ok(n.to_string())
        } else {
            Err(JsValue::from_str(&format!("Failed to get primary key, must be number or string but is: {:?}", value)))
        }
    }


    #[wasm_bindgen(js_name = matchesQuery)]
    pub fn document_matches_query(
        &self, 
        document: &JsValue, 
        query: &Query
    ) -> Result<bool, JsValue> {

        let user_query = query.get_query()?;
        let keys = Object::keys(&Object::from(user_query.clone()));

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(&user_query.clone(), &JsValue::from_str(&key))
                .map_err(|e| JsValue::from(format!("Failed to get the query value, err {:?}", e)))?;

            if key == "$and" {
                // $and operator: all conditions must be true
                if !Array::is_array(&value) {
                    return Err(JsValue::from_str("$and must be an array"));
                }
                let arr = Array::from(&value);
                for j in 0..arr.length() {
                    let item = arr.get(j);
                    let item_query = Query::new(item, query.schema.clone())?;
                    let matches = self.document_matches_query(document, &item_query)?;
                    if !matches {
                        return Ok(false);
                    }
                }
                return Ok(true);
            } else if key == "$or" {
                // $or operator: at least one condition must be true
                if !Array::is_array(&value) {
                    return Err(JsValue::from_str("$or must be an array"));
                }
                let arr = Array::from(&value);
                for j in 0..arr.length() {
                    let item = arr.get(j);
                    let item_query = Query::new(item, query.schema.clone())?;
                    let matches = self.document_matches_query(document, &item_query)?;
                    if matches {
                        return Ok(true);
                    }
                }
                return Ok(false);
            } else {
                // Attribute condition
                let doc_value = Reflect::get(document, &JsValue::from_str(&key))
                    .map_err(|e| JsValue::from(format!("Failed to get the document key, err {:?}", e)))?;

                let matches = self.evaluate_condition(&doc_value, &value)?;
                if !matches {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn evaluate_condition(&self, doc_value: &JsValue, condition: &JsValue) -> Result<bool, JsValue> {
        if condition.is_object() && !Array::is_array(condition) {
            // Condition is an object with operators
            let keys = Object::keys(&Object::from(condition.clone()));
            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let value = Reflect::get(condition, &JsValue::from_str(&key))?;
                match key.as_str() {
                    "$gt" => {
                        let res = self.compare_values(doc_value, &value, |a:f64, b:f64| a > b)?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$gte" => {
                        let res = self.compare_values(doc_value, &value, |a:f64, b:f64| a >= b)?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$lt" => {
                        let res = self.compare_values(doc_value, &value, |a:f64, b:f64| a < b)?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$lte" => {
                        let res = self.compare_values(doc_value, &value, |a:f64, b:f64| a <= b)?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$in" => {
                        if !Array::is_array(&value) {
                            return Err(JsValue::from_str("$in value must be an array"));
                        }
                        let arr = Array::from(&value);
                        let mut found = false;
                        for j in 0..arr.length() {
                            let item = arr.get(j);
                            if self.values_equal(doc_value, &item)? {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return Ok(false);
                        }
                    }
                    _ => {
                        return Err(JsValue::from_str(&format!("Unsupported operator: {}", key)));
                    }
                }
            }
            Ok(true)
        } else {
            // Direct value comparison
            self.values_equal(doc_value, condition)
        }
    }

    fn compare_values<F>(
        &self,
        doc_value: &JsValue,
        cond_value: &JsValue,
        cmp: F,
    ) -> Result<bool, JsValue>
    where
        F: Fn(f64, f64) -> bool,
    {
        let doc_num = doc_value
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Document value is not a number"))?;
        let cond_num = cond_value
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Condition value is not a number"))?;
        Ok(cmp(doc_num, cond_num))
    }

    fn values_equal(&self, doc_value: &JsValue, cond_value: &JsValue) -> Result<bool, JsValue> {
        if doc_value.is_string() && cond_value.is_string() {
            Ok(doc_value.as_string() == cond_value.as_string())
        } else if doc_value.as_f64().is_some() {
            Ok(doc_value.as_f64() == cond_value.as_f64())
        } else if doc_value.is_truthy() || cond_value.is_falsy() {
            Ok(doc_value.as_bool() == cond_value.as_bool())
        } else {
            Ok(false)
        }
    }

}
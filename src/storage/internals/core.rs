use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;

pub struct CoreStorage {

}


impl CoreStorage {

    pub(crate) fn document_matches_query(&self, document: &JsValue, query: &JsValue) -> Result<bool, JsValue> {
        // Ensure query is an object
        if !query.is_object() {
            return Err(JsValue::from_str("Query must be an object"));
        }

        let keys = Object::keys(&Object::from(query.clone()));

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(query, &JsValue::from_str(&key))
                .map_err(|e| JsValue::from(format!("Failed to get the query value")))?;

            if key == "$and" {
                // $and operator: all conditions must be true
                if !Array::is_array(&value) {
                    return Err(JsValue::from_str("$and must be an array"));
                }
                let arr = Array::from(&value);
                for j in 0..arr.length() {
                    let item = arr.get(j);
                    let matches = self.document_matches_query(document, &item)?;
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
                    let matches = self.document_matches_query(document, &item)?;
                    if matches {
                        return Ok(true);
                    }
                }
                return Ok(false);
            } else {
                // Attribute condition
                let doc_value = Reflect::get(document, &JsValue::from_str(&key))
                    .map_err(|e| JsValue::from(format!("Failed to get the document key")))?;

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
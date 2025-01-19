use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use crate::logger::Logger;
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
            let value = Reflect::get(
                &user_query.clone(),
                &JsValue::from_str(&key)
            ).map_err(|e| JsValue::from(format!("Failed to get the query value, err {:?}", e)))?;

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
                let matches = self.evaluate_condition(
                    &document,
                    key,
                    &value
                )?;
                if !matches {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn get_cmp(&self, key: String) -> Result<Box<dyn Fn(f64, f64) -> bool>, JsValue> {
        match key.as_str() {
            "$gt" => Ok(Box::new(|a, b| a > b)),
            "$gte" => Ok(Box::new(|a, b| a >= b)),
            "$lt" => Ok(Box::new(|a, b| a < b)),
            "$lte" => Ok(Box::new(|a, b| a <= b)),
            _ => Err(JsValue::from_str(&format!("Unsupported comparator: {}", key))),
        }
    }

    fn evaluate_condition(
        &self,
        document: &JsValue,
        condition_key: String,
        condition: &JsValue
    ) -> Result<bool, JsValue> {

        Logger::debug(
            "Core-evaluate-condition",
            &JsValue::from(
                &format!(
                    "Finding key {:?} in document {:?}",
                    document,
                    condition_key,
                )
            )
        );


        let document_field = Reflect::get(
            &document,
            &JsValue::from(condition_key.clone())
        )?;



        if condition.is_object() && !Array::is_array(condition) {
            // Condition is an object with operators
            let keys = Object::keys(&Object::from(condition.clone()));
            for i in 0..keys.length() {
                let key = keys.get(i).as_string().unwrap_or_default();
                let condition_value = Reflect::get(condition, &JsValue::from_str(&key.clone()))?;

                match key.as_str() {
                    "$gt" | "$gte" | "$lt" | "$lte" => {
                        let cmp = self.get_cmp(key)?;
                        let res = self.compare_values(
                            &document_field.clone(),
                            condition_key.clone(),
                            &condition_value.clone(),
                            cmp
                        )?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$in" => {
                        if !Array::is_array(&condition_value.clone()) {
                            return Err(JsValue::from_str("$in value must be an array"));
                        }
                        let arr = Array::from(&condition_value.clone());
                        let mut found = false;
                        for j in 0..arr.length() {
                            let item = arr.get(j);
                            if self.values_equal(&document_field.clone(), &item)? {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return Ok(false);
                        }
                    }
                    _ => return Err(JsValue::from_str(&format!("Unsupported operator: {}", key))),
                };
            }
            Ok(true)
        } else {
            // Direct value comparison
            self.values_equal(&document_field.clone(), condition)
        }
    }

    fn compare_values<F>(
        &self,
        doc_value: &JsValue,
        cond_key: String,
        cond_value: &JsValue,
        cmp: F,
    ) -> Result<bool, JsValue>
    where
        F: Fn(f64, f64) -> bool,
    {
        let doc_num = doc_value
            .as_f64()
            .ok_or_else(|| JsValue::from(
                format!(
                    "Document key ({:?}) is not a number but {:?} instead",
                    cond_key,
                    doc_value
                )
            ))?;

        let cond_num = cond_value
            .as_f64()
            .ok_or_else(|| JsValue::from(
                format!(
                    "Condition key ({:?}) is not a number but {:?} instead",
                    cond_key,
                    cond_value
                )
            ))?;

        let valid = cmp(doc_num, cond_num);
        Ok(
            valid
        )
    }

    fn values_equal(&self, document: &JsValue, cond_value: &JsValue) -> Result<bool, JsValue> {
        if document.is_string() && cond_value.is_string() {
            Ok(document.as_string() == cond_value.as_string())
        } else if document.as_f64().is_some() {
            Ok(document.as_f64() == cond_value.as_f64())
        } else if document.is_truthy() || cond_value.is_falsy() {
            Ok(document.as_bool() == cond_value.as_bool())
        } else {
            Ok(false)
        }
    }

}
use std::cmp::Ordering;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use crate::error::RIDBError;
use crate::operation::Operation;
use crate::query::Query;
use crate::query::options::{SortDirection, SortField};
use crate::schema::Schema;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CoreStorage {
    /**
    * @param {any} document
    * @param {Query} query
    * @returns {boolean}
    */
    matchesQuery(document: any, query: Query<any>): boolean;
    getPrimaryKeyTyped(value: any): string | number;
    getIndexes(schema: Schema<any>, op: Operation): string[];
    /**
    * Sorts a list of documents according to the provided sort specification, using the
    * same comparison semantics as the built-in storages. Intended for custom storage
    * adapters that cannot rely on a native sort. Returns a new sorted array.
    */
    sortDocuments(documents: any[], sort: SortSpec[]): any[];
}
"#;


#[derive(Debug, Clone, Copy)]
#[wasm_bindgen(skip_typescript)]
pub struct CoreStorage {

}

#[wasm_bindgen]
impl CoreStorage {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }


    #[wasm_bindgen(js_name = getPrimaryKeyTyped)]
    pub fn  get_primary_key_typed(&self, value:JsValue) -> Result<String, RIDBError> {
        if value.is_undefined() || value.is_null() {
            return Err(RIDBError::validation("Document must contain a primary key",0));
        }
        if let Some(s) = value.as_string() {
            Ok(s)
        } else if let Some(n) = value.as_f64() {
            Ok(n.to_string())
        } else {
            Err(RIDBError::validation(&format!("Failed to get primary key, must be number or string but is: {:?}", value), 0))
        }
    }


    #[wasm_bindgen(js_name = getIndexes)]
    pub fn schema_indexes(
        &self,
        schema: &Schema,
        op: &Operation
    ) -> Result<Vec<String>, RIDBError> {
        let primary_key = schema.primary_key.clone();

        let mut indexes = schema.indexes.clone()
            .unwrap_or_default()
            .into_iter()
            .map(|index| format!("idx_{}_{}", op.collection, index))
            .collect::<Vec<String>>();

        let primary_index_name = format!("pk_{}_{}", op.collection, &primary_key);

        if !indexes.contains(&primary_index_name) {
            indexes.push(primary_index_name.clone());
        }

        Ok(
            indexes
        )

    }


    /// JS-facing helper to sort an array of documents by a sort specification
    /// (`[{ field, direction }]`). Used by external storage adapters to reuse the
    /// exact ordering semantics of the built-in storages.
    #[wasm_bindgen(js_name = sortDocuments)]
    pub fn sort_documents_js(&self, documents: JsValue, sort: JsValue) -> Result<JsValue, RIDBError> {
        let sort_fields = parse_sort_fields(&sort)?;
        let docs_array = Array::from(&documents);
        let mut docs: Vec<JsValue> = docs_array.iter().collect();
        self.sort_documents(&mut docs, &sort_fields);
        let out = Array::new();
        for doc in docs {
            out.push(&doc);
        }
        Ok(out.into())
    }

    #[wasm_bindgen(js_name = matchesQuery)]
    pub fn document_matches_query(
        &self, 
        document: &JsValue, 
        query: &Query
    ) -> Result<bool, RIDBError> {

        if !document.is_object() {
            return Ok(false);
        }

        let user_query = query.get_query()?;
        let keys = Object::keys(&Object::from(user_query.clone()));

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap_or_default();
            let value = Reflect::get(&user_query.clone(), &JsValue::from_str(&key))
                .map_err(|e| JsValue::from(format!("Failed to get the query value, err {:?}", e)))?;

            if key == "$and" {
                // $and operator: all conditions must be true
                if !Array::is_array(&value) {
                    return Err(RIDBError::validation("$and must be an array", 0));
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
                    return Err(RIDBError::validation("$or must be an array", 0));
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
                let matches = self.evaluate_condition(document, key, &value)?;
                if !matches {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn get_cmp(&self, key: String) -> Result<Box<dyn Fn(f64, f64) -> bool>, RIDBError> {
        match key.as_str() {
            "$gt" => Ok(Box::new(|a, b| a > b)),
            "$gte" => Ok(Box::new(|a, b| a >= b)),
            "$lt" => Ok(Box::new(|a, b| a < b)),
            "$lte" => Ok(Box::new(|a, b| a <= b)),
            _ => Err(RIDBError::validation(&format!("Unsupported comparator: {}", key), 0)),
        }
    }

    fn evaluate_condition(
        &self,
        document: &JsValue,
        condition_key: String,
        condition: &JsValue
    ) -> Result<bool, RIDBError> {
        let document_field = Reflect::get(document, &JsValue::from(condition_key.clone()))?;

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
                            &document_field,
                            condition_key.clone(),
                            &condition_value,
                            cmp
                        )?;
                        if !res {
                            return Ok(false);
                        }
                    }
                    "$in" => {
                        if !Array::is_array(&condition_value) {
                            return Err(RIDBError::validation("$in value must be an array",0));
                        }
                        let arr = Array::from(&condition_value);
                        let mut found = false;
                        for j in 0..arr.length() {
                            let item = arr.get(j);
                            if self.values_equal(&document_field, &item)? {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return Ok(false);
                        }
                    }
                    "$nin" => {
                        if !Array::is_array(&condition_value) {
                            return Err(RIDBError::validation("$nin value must be an array", 0));
                        }
                        let arr = Array::from(&condition_value);
                        for j in 0..arr.length() {
                            let item = arr.get(j);
                            let valid = self.values_equal(&document_field, &item)?;
                            if valid {
                                return Ok(false);
                            }
                        }
                    }
                    "$eq" => {
                        // $eq operator: must equal the condition
                        let eq = self.values_equal(&document_field, &condition_value)?;
                        if !eq {
                            return Ok(false);
                        }
                    }
                    "$ne" => {
                        // $ne operator: must NOT equal the condition
                        let eq = self.values_equal(&document_field, &condition_value)?;
                        if eq {
                            return Ok(false);
                        }
                    }
                    _ => {
                        return Err(RIDBError::validation(
                            &format!("Unsupported operator: {}", key),
                            0
                        ));
                    }
                };
            }
            Ok(true)
        } else {
            // Direct value comparison
            self.values_equal(&document_field, condition)
        }
    }

    fn compare_values<F>(
        &self,
        doc_value: &JsValue,
        cond_key: String,
        cond_value: &JsValue,
        cmp: F,
    ) -> Result<bool, RIDBError>
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

    fn values_equal(&self, document: &JsValue, cond_value: &JsValue) -> Result<bool, RIDBError> {
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

/// Parses a JS sort specification (`[{ field: string, direction?: 'asc' | 'desc' }]`)
/// into the internal `Vec<SortField>` representation. Accepts `undefined`/`null` as
/// "no sorting" and defaults an omitted/unknown direction to ascending.
fn parse_sort_fields(sort: &JsValue) -> Result<Vec<SortField>, RIDBError> {
    let mut result = Vec::new();
    if sort.is_undefined() || sort.is_null() {
        return Ok(result);
    }
    if !Array::is_array(sort) {
        return Err(RIDBError::validation("sort must be an array of sort specifications", 0));
    }
    let arr = Array::from(sort);
    for i in 0..arr.length() {
        let item = arr.get(i);
        let field = Reflect::get(&item, &JsValue::from_str("field"))?
            .as_string()
            .ok_or_else(|| RIDBError::validation("sort 'field' must be a string", 0))?;
        let direction = Reflect::get(&item, &JsValue::from_str("direction"))?
            .as_string()
            .map(|d| match d.to_lowercase().as_str() {
                "desc" => SortDirection::Desc,
                _ => SortDirection::Asc,
            })
            .unwrap_or(SortDirection::Asc);
        result.push(SortField { field, direction });
    }
    Ok(result)
}

impl CoreStorage {
    /// Sorts a list of documents in place according to the provided sort fields.
    /// Fields are applied in order (the first field is primary, subsequent ones break ties).
    /// A no-op when `sort` is empty.
    pub(crate) fn sort_documents(&self, documents: &mut [JsValue], sort: &[SortField]) {
        if sort.is_empty() {
            return;
        }
        documents.sort_by(|a, b| Self::compare_by_sort(a, b, sort));
    }

    fn compare_by_sort(a: &JsValue, b: &JsValue, sort: &[SortField]) -> Ordering {
        for field in sort {
            let a_value = Reflect::get(a, &JsValue::from_str(&field.field)).unwrap_or(JsValue::UNDEFINED);
            let b_value = Reflect::get(b, &JsValue::from_str(&field.field)).unwrap_or(JsValue::UNDEFINED);
            let mut ordering = Self::compare_field_values(&a_value, &b_value);
            if field.direction == SortDirection::Desc {
                ordering = ordering.reverse();
            }
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    }

    /// Compares two field values for ordering purposes. Numbers are compared numerically,
    /// strings lexicographically, and booleans with `false < true`. Missing values
    /// (`undefined`/`null`) always sort after present values (in ascending order).
    fn compare_field_values(a: &JsValue, b: &JsValue) -> Ordering {
        let a_missing = a.is_undefined() || a.is_null();
        let b_missing = b.is_undefined() || b.is_null();
        match (a_missing, b_missing) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            (false, false) => {}
        }

        if let (Some(a_num), Some(b_num)) = (a.as_f64(), b.as_f64()) {
            return a_num.partial_cmp(&b_num).unwrap_or(Ordering::Equal);
        }

        if a.is_string() || b.is_string() {
            let a_str = a.as_string().unwrap_or_default();
            let b_str = b.as_string().unwrap_or_default();
            return a_str.cmp(&b_str);
        }

        if let (Some(a_bool), Some(b_bool)) = (a.as_bool(), b.as_bool()) {
            return a_bool.cmp(&b_bool);
        }

        Ordering::Equal
    }
}
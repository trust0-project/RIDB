use js_sys::{Array, Promise, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::JsFuture;
use crate::utils::Logger;
use crate::query::Query;
use crate::storage::internals::core::CoreStorage;
use web_sys::{IdbDatabase, IdbFactory, IdbIndexParameters, IdbKeyRange, IdbOpenDbRequest, IdbRequest};
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use crate::error::RIDBError;
use crate::schema::Schema;


pub fn get_key_range(value: &JsValue) -> Result<Option<IdbKeyRange>, RIDBError> {
    if !value.is_object() || Array::is_array(value) {
        // This is a direct value, e.g. "age": 30
        return Ok(Some(IdbKeyRange::only(value)?));
    }

    let obj = Object::from(value.clone());
    let keys = Object::keys(&obj);

    if keys.length() == 1 {
        let key = keys.get(0).as_string().unwrap_or_default();
        let val = Reflect::get(&obj, &keys.get(0))?;
        match key.as_str() {
            "$eq" => return Ok(Some(IdbKeyRange::only(&val)?)),
            "$gt" => return Ok(Some(IdbKeyRange::lower_bound_with_open(&val, true)?)),
            "$gte" => return Ok(Some(IdbKeyRange::lower_bound(&val)?)),
            "$lt" => return Ok(Some(IdbKeyRange::upper_bound_with_open(&val, true)?)),
            "$lte" => return Ok(Some(IdbKeyRange::upper_bound(&val)?)),
            "$in" | "$nin" => return Ok(None), // IdbKeyRange doesn't support $in directly, cursor iteration is needed for each value
            _ => return Ok(None),
        }
    }

    if keys.length() == 2 {
        let key1_js = keys.get(0);
        let key2_js = keys.get(1);
        let key1 = key1_js.as_string().unwrap_or_default();
        let key2 = key2_js.as_string().unwrap_or_default();
        let val1 = Reflect::get(&obj, &key1_js)?;
        let val2 = Reflect::get(&obj, &key2_js)?;

        let mut lower: Option<JsValue> = None;
        let mut upper: Option<JsValue> = None;
        let mut lower_open = false;
        let mut upper_open = false;

        for (k, v) in [(key1.as_str(), val1), (key2.as_str(), val2)] {
            match k {
                "$gt" => { lower = Some(v); lower_open = true; },
                "$gte" => { lower = Some(v); lower_open = false; },
                "$lt" => { upper = Some(v); upper_open = true; },
                "$lte" => { upper = Some(v); upper_open = false; },
                _ => return Ok(None),
            }
        }

        if let (Some(l), Some(u)) = (lower, upper) {
            return Ok(Some(IdbKeyRange::bound_with_lower_open_and_upper_open(&l, &u, lower_open, upper_open)?));
        }
    }
    
    Ok(None)
}

pub async fn get_pks_from_index(index: &web_sys::IdbIndex, range: &JsValue) -> Result<HashSet<String>, RIDBError> {
    let pks = Arc::new(parking_lot::Mutex::new(HashSet::new()));
    let request = if range.is_undefined() || range.is_null() {
        index.open_key_cursor()
    } else {
        index.open_key_cursor_with_range(range)
    }?;

    let promise = Promise::new(&mut |resolve, reject| {
        let pks_clone = pks.clone();
        let reject_clone = reject.clone();

        let onsuccess = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let result = (|| {
                let target = event.target().ok_or_else(|| JsValue::from_str("No event target"))?;
                let request = target.dyn_into::<IdbRequest>().map_err(|_| JsValue::from_str("Event target is not an IdbRequest"))?;
                request.result()
            })();

            match result {
                Ok(cursor_val) if !cursor_val.is_null() => {
                    if let Ok(cursor) = cursor_val.dyn_into::<web_sys::IdbCursor>() {
                        if let Ok(pk) = cursor.primary_key() {
                            if let Some(pk_str) = pk.as_string() {
                                pks_clone.lock().insert(pk_str);
                            } else if let Some(pk_num) = pk.as_f64() {
                                pks_clone.lock().insert(pk_num.to_string());
                            }
                        }
                        if let Err(e) = cursor.continue_() {
                            let _ = reject_clone.call1(&JsValue::NULL, &e);
                        }
                    } else {
                        let _ = reject_clone.call1(&JsValue::NULL, &JsValue::from_str("Result is not a cursor"));
                    }
                },
                Ok(_) => { // cursor_val is null
                    let _ = resolve.call1(&JsValue::NULL, &JsValue::NULL);
                },
                Err(e) => {
                    let _ = reject_clone.call1(&JsValue::NULL, &e);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let onerror = Closure::once(Box::new(move |event: web_sys::Event| {
            let _ = reject.call1(&JsValue::NULL, &event);
        }));

        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onsuccess.forget();
        // Ensure the error handler is kept alive for the lifetime of the request as well
        onerror.forget();
    });

    JsFuture::from(promise).await?;
    let final_pks = pks.lock().clone();
    Ok(final_pks)
}


pub async fn cursor_fetch_and_filter(
    index: Option<&web_sys::IdbIndex>,
    store: Option<&web_sys::IdbObjectStore>,
    range: &JsValue,
    core: CoreStorage,
    value_query: Query,
    offset: u32,
    limit: u32,
) -> Result<Array, RIDBError> {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Use Rc<RefCell<>> for shared state between closures
    let all_docs = Rc::new(RefCell::new(Vec::new()));
    let matched_count = Rc::new(RefCell::new(0u32));
    let skipped_count = Rc::new(RefCell::new(0u32));
    
    let promise = Promise::new(&mut |resolve, reject| {
        let all_docs_ref = all_docs.clone();
        let matched_count_ref = matched_count.clone();
        let skipped_count_ref = skipped_count.clone();
        let resolve_ref = resolve.clone();
        let reject_ref = reject.clone();
        let value_query_clone = value_query.clone();

        // Create a lightweight success handler that collects documents with early termination
        let on_success = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            let target_res = evt.target().and_then(|t| t.dyn_into::<web_sys::IdbRequest>().ok());
            if target_res.is_none() {
                let _ = reject_ref.call1(&JsValue::NULL, &JsValue::from_str("Failed to get IdbRequest from event."));
                return;
            }
            let target = target_res.unwrap();
        
            let cursor_value_res = target.result();
            if cursor_value_res.is_err() || cursor_value_res.as_ref().map_or(true, |v| v.is_null() || v.is_undefined()) {
                let result_array = Array::from_iter(all_docs_ref.borrow().iter());
                let _ = resolve_ref.call1(&JsValue::NULL, &result_array);
                return;
            }
            let cursor_value = cursor_value_res.unwrap();
            
            let cursor_res = cursor_value.dyn_into::<web_sys::IdbCursorWithValue>();
            if cursor_res.is_err() {
                let _ = reject_ref.call1(&JsValue::NULL, &JsValue::from_str("Failed to cast to IdbCursorWithValue."));
                return;
            }
            let cursor = cursor_res.unwrap();
        
            if *matched_count_ref.borrow() >= limit {
                let result_array = Array::from_iter(all_docs_ref.borrow().iter());
                let _ = resolve_ref.call1(&JsValue::NULL, &result_array);
                return;
            }
        
            if let Ok(doc) = cursor.value() {
                if core.document_matches_query(&doc, &value_query_clone).unwrap_or(false) {
                    let mut skipped = skipped_count_ref.borrow_mut();
                    let mut matched = matched_count_ref.borrow_mut();
                    
                    if *skipped < offset {
                        *skipped += 1;
                    } else if *matched < limit {
                        all_docs_ref.borrow_mut().push(doc);
                        *matched += 1;
                    }
                    
                    if *matched >= limit {
                        let result_array = Array::from_iter(all_docs_ref.borrow().iter());
                        let _ = resolve_ref.call1(&JsValue::NULL, &result_array);
                        return;
                    }
                }
                
                if let Err(err) = cursor.continue_() {
                    let _ = reject_ref.call1(&JsValue::NULL, &err);
                }
            } else if let Err(err) = cursor.value() {
                let _ = reject_ref.call1(&JsValue::NULL, &err);
            }
        }) as Box<dyn FnMut(_)>);

        let reject_err = reject.clone();
        let on_error = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            let _ = reject_err.call1(&JsValue::NULL, &evt);
        }) as Box<dyn FnMut(_)>);

        let request_result = if let Some(idx) = index {
            if range.is_undefined() || range.is_null() {
                idx.open_cursor()
            } else {
                idx.open_cursor_with_range(range)
            }
        } else if let Some(st) = store {
            if range.is_undefined() || range.is_null() {
                st.open_cursor()
            } else {
                st.open_cursor_with_range(range)
            }
        } else {
            Err(JsValue::from_str("No index or store provided to open cursor."))
        };

        match request_result {
            Ok(request) => {
                request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
                request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

                on_success.forget();
                on_error.forget();
            }
            Err(e) => {
                let _ = reject.call1(&JsValue::NULL, &e);
            }
        }
    });

    let js_result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(Array::from(&js_result))
}

pub async fn idb_request_result(request: IdbRequest) -> Result<JsValue, JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let reject_clone = reject.clone();
        let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
            let result = event
                .target()
                .ok_or_else(|| JsValue::from_str("No event target"))
                .and_then(|t| {
                    t.dyn_into::<IdbRequest>()
                        .map_err(|_| JsValue::from_str("Event target is not an IdbRequest"))
                })
                .and_then(|req| req.result());

            match result {
                Ok(value) => {
                    let _ = resolve.call1(&JsValue::NULL, &value);
                }
                Err(e) => {
                    let _ = reject.call1(&JsValue::NULL, &e);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let onerror = Closure::once(Box::new(move |event: web_sys::Event| {
            let error = event
                .target()
                .and_then(|t| t.dyn_into::<IdbRequest>().ok())
                .and_then(|req| req.error().ok().flatten())
                .map(JsValue::from)
                .unwrap_or_else(|| JsValue::from_str("Unknown error"));
            let _ = reject_clone.call1(&JsValue::NULL, &error);
        }) as Box<dyn FnMut(_)>);

        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onsuccess.forget();
        onerror.forget();
    });

    JsFuture::from(promise).await
}

pub fn get_indexed_db() -> Result<IdbFactory, RIDBError> {
    // 1) If in a normal browser (Window) environment
    if let Ok(window) = js_sys::global().dyn_into::<web_sys::Window>() {
        if let Some(idb) = window.indexed_db()? {
            return Ok(idb);
        }
    }
    // 2) If in a Worker context
    else if let Ok(worker) = js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>() {
        if let Some(idb) = worker.indexed_db()? {
            return Ok(idb);
        }
    }

    Err(RIDBError::from("IndexedDB not available in this environment"))
}

pub fn get_indexed_fields_in_query(
    query: &Query,
    schema: &Schema
) -> Result<Vec<String>, RIDBError> {
    let fields = query.get_properties()?;
    let schema_indexes = &schema.indexes;
    let mut indexed_fields = Vec::new();
    if let Some(indexes) = schema_indexes {
        for index in indexes {
            if fields.contains(index) {
                indexed_fields.push(index.clone());
            }
        }
    }
    Ok(indexed_fields)
}


pub async fn create_database(name: &str, schemas: HashMap<String, Schema>) -> Result<Arc<IdbDatabase>, RIDBError> {
    let idb = get_indexed_db()?;

    let version = 1;
    let db_request = idb.open_with_u32(name, version)?;

    // Clone keys before entering the Promise

    let keys_vec: Vec<String> = schemas.keys()
        .map(|k| k.to_string())
        .collect();

    let db = JsFuture::from(Promise::new(&mut |resolve, reject| {
        let keys = keys_vec.clone();
        let schemas_clone = schemas.clone();
        let onupgradeneeded = Closure::once(Box::new(move |event: web_sys::Event| {
            let db: IdbDatabase = event.target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap()
                .result()
                .unwrap()
                .dyn_into()
                .unwrap();

            for collection_name in keys {
                let schema = schemas_clone.get(&collection_name).unwrap();
                if !db.object_store_names().contains(&collection_name) {
                    // Create object store
                    let object_store = db
                        .create_object_store(&collection_name)
                        .expect("Failed to create object store");

                    // If there are indexes, create them
                    if let Some(indexes) = &schema.indexes {
                        for index_name in indexes {
                            let mut index_params = IdbIndexParameters::new();
                            index_params.unique(false);
                            index_params.multi_entry(false);
                            Logger::debug(
                                "IndexDB",
                                &JsValue::from(
                                    format!(
                                        "Creating index in collection {} ::: {}",
                                        &collection_name,
                                        index_name
                                    )
                                )
                            );
                            object_store
                                .create_index_with_str_and_optional_parameters(
                                    index_name, // index name
                                    index_name, // key path
                                    &index_params,
                                )
                                .expect("Failed to create index");
                        }
                    }
                }
            }
        }));

        let onsuccess = Closure::once(Box::new(move |event: web_sys::Event| {
            let db: IdbDatabase = event.target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap()
                .result()
                .unwrap()
                .dyn_into()
                .unwrap();
            resolve.call1(&JsValue::undefined(), &db).unwrap();
        }));

        let onerror = Closure::once(Box::new(move |e: web_sys::Event| {
            reject.call1(&JsValue::undefined(), &e).unwrap();
        }));

        db_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        db_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        db_request.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onupgradeneeded.forget();
        onsuccess.forget();
        onerror.forget();
    })).await?;

    Ok(Arc::new(db.dyn_into::<IdbDatabase>()?))
}
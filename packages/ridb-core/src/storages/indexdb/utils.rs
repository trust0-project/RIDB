use js_sys::{Array, Promise};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::JsFuture;
use crate::utils::Logger;
use crate::query::Query;
use crate::storage::internals::core::CoreStorage;
use web_sys::{IdbDatabase, IdbFactory, IdbIndexParameters, IdbKeyRange, IdbOpenDbRequest, IdbRequest};
use std::sync::Arc;
use std::collections::HashMap;
use crate::error::RIDBError;
use crate::schema::Schema;


pub async fn cursor_fetch_and_filter(
    index: Option<&web_sys::IdbIndex>,
    store: Option<&web_sys::IdbObjectStore>,
    key_value: &JsValue,
    core: CoreStorage,
    value_query: Query,
    offset: u32,
    limit: u32,
) -> Result<Array, RIDBError> {
    use std::cell::RefCell;

    let result_array = RefCell::new(Array::new());
    let skipped_count = RefCell::new(0u32);
    let matched_count = RefCell::new(0u32);
    
    // Clone these before creating the promise to avoid ownership issues
    let core_cloned = core.clone();
    let value_query_cloned = value_query.clone();

    let promise = Promise::new(&mut |resolve, reject| {
        // Create references to the arrays and counters
        let result_array_ref = result_array.clone();
        let skipped_count_ref = skipped_count.clone();
        let matched_count_ref = matched_count.clone();
        let value_query_ref = value_query_cloned.clone();
        
        // References to resolver/rejecter
        let resolve_ref = resolve.clone();
        let reject_ref = reject.clone();

        // Create closures with owned values
        let on_success = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            let target: web_sys::IdbRequest = match evt.target().and_then(|t| t.dyn_into().ok()) {
                Some(req) => req,
                None => {
                    let _ = reject_ref.call1(
                        &JsValue::NULL,
                        &JsValue::from_str("Failed to cast event target to IdbRequest."),
                    );
                    return;
                }
            };

            let cursor_value = target.result();
            if cursor_value.is_err()
                || cursor_value.as_ref().unwrap().is_null()
                || cursor_value.as_ref().unwrap().is_undefined()
            {
                // Cursor finished: resolve with the final array
                let _ = resolve_ref.call1(
                    &JsValue::NULL,
                    &result_array_ref.borrow(),
                );
                return;
            }

            let cursor: web_sys::IdbCursorWithValue = match cursor_value.unwrap().dyn_into() {
                Ok(c) => c,
                Err(_) => {
                    let _ = reject_ref.call1(
                        &JsValue::NULL,
                        &JsValue::from_str("Failed to cast cursor to IdbCursorWithValue."),
                    );
                    return;
                }
            };

            let doc = match cursor.value() {
                Ok(val) => val,
                Err(err) => {
                    let _ = reject_ref.call1(&JsValue::NULL, &err);
                    return;
                }
            };

            // Filter in-memory based on the original query
            if core_cloned
                .document_matches_query(&doc, value_query_ref.clone())
                .unwrap_or(false)
            {
                let mut skip_ref = skipped_count_ref.borrow_mut();
                let mut match_ref = matched_count_ref.borrow_mut();

                if *skip_ref < offset {
                    *skip_ref += 1;
                } else if *match_ref < limit {
                    result_array_ref.borrow().push(&doc);
                    *match_ref += 1;
                }
                if *match_ref >= limit {
                    // Found enough docs: resolve immediately
                    let _ = resolve_ref.call1(
                        &JsValue::NULL,
                        &result_array_ref.borrow(),
                    );
                    return;
                }
            }

            // Advance cursor
            if let Err(err) = cursor.continue_() {
                let _ = reject_ref.call1(&JsValue::NULL, &err);
            }
        }) as Box<dyn FnMut(_)>);

        // Create another reject_ref for the error handler
        let reject_err = reject.clone();
        let on_error = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            let _ = reject_err.call1(&JsValue::NULL, &evt);
        }) as Box<dyn FnMut(_)>);

        // Decide how to open the cursor
        let request_result = if let Some(idx) = index {
            if !key_value.is_null() && !key_value.is_undefined() {
                match IdbKeyRange::only(key_value) {
                    Ok(range) => idx.open_cursor_with_range(&range),
                    Err(_) => idx.open_cursor(),
                }
            } else {
                idx.open_cursor()
            }
        } else if let Some(st) = store {
            if !key_value.is_null() && !key_value.is_undefined() {
                match IdbKeyRange::only(key_value) {
                    Ok(range) => st.open_cursor_with_range(&range),
                    Err(_) => st.open_cursor(),
                }
            } else {
                st.open_cursor()
            }
        } else {
            Err(JsValue::from_str("No index or store provided to open cursor."))
        };

        // Attach success/error closures to the request
        match request_result {
            Ok(request) => {
                request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
                request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

                // Keep the closures alive for multiple invocations
                on_success.forget();
                on_error.forget();
            }
            Err(e) => {
                let _ = reject.call1(&JsValue::NULL, &e);
            }
        }
    });

    // Await the promise, then convert the result to an Array
    let js_result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(Array::from(&js_result))
}


pub async fn idb_request_result(request: IdbRequest) -> Result<JsValue, JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let reject2 = reject.clone();
        let success_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            let request: IdbRequest = event.target()
                .unwrap()
                .dyn_into()
                .unwrap();

            match request.result() {
                Ok(result) => resolve.call1(&JsValue::undefined(), &result).unwrap(),
                Err(e) => reject.call1(&JsValue::undefined(), &e).unwrap(),
            }
        }));

        let error_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            let request: IdbRequest = event.target()
                .unwrap()
                .dyn_into()
                .unwrap();

            let error = request.error().unwrap();
            reject2.call1(&JsValue::undefined(), &error.unwrap()).unwrap();
        }));

        request.set_onsuccess(Some(success_callback.as_ref().unchecked_ref()));
        request.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        // The closures will automatically be dropped after the Promise resolves/rejects
        success_callback.forget();
        error_callback.forget();
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

pub fn can_use_single_index_lookup(
    query: Query,
    schema: Schema
) -> Result<Option<String>, RIDBError> {
    let fields = query.get_properties()?;
    let schema_indexes = &schema.indexes;
    if let Some(indexes) = schema_indexes {
        for index in indexes {
            if fields.contains(index) {
                return Ok(
                    Some(
                        index.clone()
                    )
                )
            }
        }
    }
    Ok(
        None
    )
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
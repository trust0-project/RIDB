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
    use std::rc::Rc;

    // Use Rc<RefCell<>> for shared state between closures
    let all_docs = Rc::new(RefCell::new(Vec::new()));
    let cursor_finished = Rc::new(RefCell::new(false));
    let matched_count = Rc::new(RefCell::new(0u32));
    let skipped_count = Rc::new(RefCell::new(0u32));
    let processing_scheduled = Rc::new(RefCell::new(false));
    
    // Clone these for the async processing
    let core_cloned = core.clone();
    let value_query_cloned = value_query.clone();

    let promise = Promise::new(&mut |resolve, reject| {
        let all_docs_ref = all_docs.clone();
        let cursor_finished_ref = cursor_finished.clone();
        let matched_count_ref = matched_count.clone();
        let skipped_count_ref = skipped_count.clone();
        let processing_scheduled_ref = processing_scheduled.clone();
        let resolve_ref = resolve.clone();
        let reject_ref = reject.clone();
        let value_query_for_closure = value_query_cloned.clone();
        let core_for_closure = core_cloned.clone();

        // Create a lightweight success handler that collects documents with early termination
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
            
            // If cursor is done, start async processing
            if cursor_value.is_err()
                || cursor_value.as_ref().unwrap().is_null()
                || cursor_value.as_ref().unwrap().is_undefined()
            {
                *cursor_finished_ref.borrow_mut() = true;
                // Only schedule async processing if it hasn't been scheduled yet
                if !*processing_scheduled_ref.borrow() {
                    *processing_scheduled_ref.borrow_mut() = true;
                    // Defer the heavy processing to avoid blocking the success handler
                    schedule_async_processing(
                        all_docs_ref.clone(),
                        core_for_closure.clone(),
                        value_query_for_closure.clone(),
                        offset,
                        limit,
                        resolve_ref.clone(),
                        reject_ref.clone(),
                    );
                }
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

            // Check if we've already collected enough documents
            let current_matched = *matched_count_ref.borrow();
            if current_matched >= limit {
                // We have enough matches, stop cursor iteration
                *cursor_finished_ref.borrow_mut() = true;
                // Only schedule async processing if it hasn't been scheduled yet
                if !*processing_scheduled_ref.borrow() {
                    *processing_scheduled_ref.borrow_mut() = true;
                    schedule_async_processing(
                        all_docs_ref.clone(),
                        core_for_closure.clone(),
                        value_query_for_closure.clone(),
                        offset,
                        limit,
                        resolve_ref.clone(),
                        reject_ref.clone(),
                    );
                }
                return;
            }

            // Lightweight work: collect the document with early termination logic
            match cursor.value() {
                Ok(doc) => {
                    // Quick check for query match to enable early termination
                    if core_for_closure.document_matches_query(&doc, value_query_for_closure.clone()).unwrap_or(false) {
                        let mut skipped = skipped_count_ref.borrow_mut();
                        let mut matched = matched_count_ref.borrow_mut();
                        
                        if *skipped < offset {
                            *skipped += 1;
                        } else if *matched < limit {
                            all_docs_ref.borrow_mut().push(doc);
                            *matched += 1;
                        }
                        
                        // If we've reached the limit, stop processing
                        if *matched >= limit {
                            *cursor_finished_ref.borrow_mut() = true;
                            // Only schedule async processing if it hasn't been scheduled yet
                            if !*processing_scheduled_ref.borrow() {
                                *processing_scheduled_ref.borrow_mut() = true;
                                schedule_async_processing(
                                    all_docs_ref.clone(),
                                    core_for_closure.clone(),
                                    value_query_for_closure.clone(),
                                    offset,
                                    limit,
                                    resolve_ref.clone(),
                                    reject_ref.clone(),
                                );
                            }
                            return;
                        }
                    }
                    
                    // Continue cursor to next record
                    if let Err(err) = cursor.continue_() {
                        let _ = reject_ref.call1(&JsValue::NULL, &err);
                    }
                },
                Err(err) => {
                    let _ = reject_ref.call1(&JsValue::NULL, &err);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let reject_err = reject.clone();
        let on_error = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            let _ = reject_err.call1(&JsValue::NULL, &evt);
        }) as Box<dyn FnMut(_)>);

        // Open cursor logic remains the same
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

// Schedule async processing using setTimeout to avoid blocking the event loop
// Fixed to work in both Window and Worker contexts
fn schedule_async_processing(
    all_docs: std::rc::Rc<std::cell::RefCell<Vec<JsValue>>>,
    core: CoreStorage,
    query: Query,
    offset: u32,
    limit: u32,
    resolve: js_sys::Function,
    reject: js_sys::Function,
) {
    let timeout_callback = Closure::once(Box::new(move || {
        match process_documents_async(all_docs, core, query, offset, limit) {
            Ok(result) => {
                let _ = resolve.call1(&JsValue::NULL, &result);
            }
            Err(e) => {
                let _ = reject.call1(&JsValue::NULL, &e);
            }
        }
    }));

    // Use setTimeout in both Window and Worker contexts
    if let Ok(window) = js_sys::global().dyn_into::<web_sys::Window>() {
        // Window context
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_callback.as_ref().unchecked_ref(),
            0, // Next tick
        );
    } else if let Ok(worker) = js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>() {
        // Worker context
        let _ = worker.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_callback.as_ref().unchecked_ref(),
            0, // Next tick
        );
    } else {
        // Fallback: execute immediately if neither context is available
        // This shouldn't happen in normal web environments, but provides a safety net
        let callback_fn = timeout_callback.as_ref().unchecked_ref::<js_sys::Function>();
        let _ = callback_fn.call0(&JsValue::undefined());
    }
    
    timeout_callback.forget();
}

// Process documents asynchronously - now mainly for final array construction
// since filtering and limits are already applied during cursor iteration
fn process_documents_async(
    all_docs: std::rc::Rc<std::cell::RefCell<Vec<JsValue>>>,
    _core: CoreStorage,
    _query: Query,
    _offset: u32,
    _limit: u32,
) -> Result<Array, JsValue> {
    let docs = all_docs.borrow();
    let result_array = Array::new();
    
    // Since filtering and limits are already applied during cursor iteration,
    // we just need to convert the collected documents to an Array
    for doc in docs.iter() {
        result_array.push(doc);
    }
    
    Ok(result_array)
}

pub async fn idb_request_result(request: IdbRequest) -> Result<JsValue, JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let reject2 = reject.clone();
        
        // Ultra-lightweight success handler that just resolves immediately
        let success_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            // Minimize work in success handler - just get the target and resolve
            if let Some(target) = event.target() {
                if let Ok(request) = target.dyn_into::<IdbRequest>() {
                    // Get result and resolve immediately without additional processing
                    match request.result() {
                        Ok(result) => {
                            let _ = resolve.call1(&JsValue::undefined(), &result);
                        }
                        Err(e) => {
                            let _ = reject.call1(&JsValue::undefined(), &e);
                        }
                    }
                } else {
                    let _ = reject.call1(&JsValue::undefined(), &JsValue::from_str("Invalid request target"));
                }
            } else {
                let _ = reject.call1(&JsValue::undefined(), &JsValue::from_str("No event target"));
            }
        }));

        // Ultra-lightweight error handler
        let error_callback = Closure::once(Box::new(move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Ok(request) = target.dyn_into::<IdbRequest>() {
                    // Get error and reject immediately
                    match request.error() {
                        Ok(Some(error)) => {
                            let _ = reject2.call1(&JsValue::undefined(), &error);
                        }
                        Ok(None) => {
                            let _ = reject2.call1(&JsValue::undefined(), &JsValue::from_str("Unknown error"));
                        }
                        Err(e) => {
                            let _ = reject2.call1(&JsValue::undefined(), &e);
                        }
                    }
                } else {
                    let _ = reject2.call1(&JsValue::undefined(), &JsValue::from_str("Invalid request target"));
                }
            } else {
                let _ = reject2.call1(&JsValue::undefined(), &JsValue::from_str("No event target"));
            }
        }));

        request.set_onsuccess(Some(success_callback.as_ref().unchecked_ref()));
        request.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

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
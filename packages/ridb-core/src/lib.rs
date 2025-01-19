use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::panic;

pub mod error;
pub mod utils;
pub mod schema;
pub mod collection;
pub mod storages;
pub mod storage;
pub mod database;
pub mod query;
pub mod operation;
pub mod plugin;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn is_debug_mode() -> bool {
    get_debug_mode()
}

fn get_debug_mode() -> bool {
    use wasm_bindgen::prelude::*;
    use js_sys::Reflect;

    if let Some(win) = web_sys::window() {
        // Browser environment
        win.local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item("DEBUG").ok().flatten())
            .map(|debug_str| {
                debug_str
                    .split(',')
                    .any(|s| s == "ridb" || s.starts_with("ridb:*"))
            })
            .unwrap_or(false)
    } else {
        // Node.js environment
        // Access process.env.DEBUG directly
        let global = js_sys::global();

        let process = Reflect::get(&global, &JsValue::from_str("process")).ok();
        let env = process
            .as_ref()
            .and_then(|proc| Reflect::get(proc, &JsValue::from_str("env")).ok());
        let debug_var = env
            .as_ref()
            .and_then(|env| Reflect::get(env, &JsValue::from_str("DEBUG")).ok());

        if let Some(debug_js_value) = debug_var {
            if let Some(debug_str) = debug_js_value.as_string() {
                debug_str
                    .split(',')
                    .any(|s| s == "ridb" || s.starts_with("ridb:*"))
            } else {
                false
            }
        } else {
            false
        }
    }
}

mod logger {
    use wasm_bindgen::prelude::*;
    use web_sys::console;

    pub struct Logger;

    impl Logger {
        pub fn error(component: &str, message: &JsValue) {
            Logger::err_1(component, message);
        }
        pub fn log(component: &str, message: &JsValue) {
            Logger::log_1(component, message);
        }
        pub fn debug(component: &str, message: &JsValue) {
            if crate::is_debug_mode() {
                Logger::log_1(component, message);
            }
        }

        fn log_1(component: &str, message: &JsValue) {
            console::log_1(
                &JsValue::from(
                    format!("[{}] {:?}", component, message)
                )
            );
        }

        fn err_1(component: &str, message: &JsValue) {
            console::log_1(
                &JsValue::from(
                    format!("[{}] {:?}", component, message)
                )
            );
        }
    }
}
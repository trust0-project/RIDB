use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

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
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn is_debug_mode() -> bool {
    get_debug_mode()
}

fn get_debug_mode() -> bool {
    if let Some(win) = web_sys::window() {
        return win.local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item("DEBUG").ok().flatten())
            .map(|debug_str| {
                debug_str
                    .split(',')
                    .any(|s| s == "ridb" || s.starts_with("ridb:*"))
            })
            .unwrap_or(false);
    }

    return std::env::var("DEBUG")
        .ok()
        .map(|debug_var| {
            debug_var
                .split(',')
                .any(|s| s == "ridb" || s.starts_with("ridb:*"))
        })
        .unwrap_or(false);
}



mod logger {
    use wasm_bindgen::prelude::*;
    use web_sys::console;

    pub struct Logger;

    impl Logger {
        pub fn log(message: &JsValue) {
            Logger::log_1(message);
        }

        pub fn debug(message: &JsValue) {
            if crate::is_debug_mode() {
                Logger::log_1(message);
            }
        }

        fn log_1(message: &JsValue) {
            console::log_1(&message);
        }
    }
}
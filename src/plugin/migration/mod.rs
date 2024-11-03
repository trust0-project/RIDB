use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::plugin::BasePlugin;
use crate::schema::Schema;
use js_sys::{Object, Reflect};
use wasm_bindgen::__rt::IntoJsResult;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
type EnumerateUpTo < N extends number, Acc extends number[] = [] > = Acc['length'] extends N ?
    Acc[number] :
    EnumerateUpTo < N, [...Acc, Acc['length']] > ;

type EnumerateFrom2To < N extends number > = Exclude < EnumerateUpTo < N > , 0 | 1 > | (N extends 0 | 1 ? never : N);

type IsVersionGreaterThan1 < V extends number > = V extends 0 | 1 ? false : true;
type AnyVersionGreaterThan1 < T extends Record < string, SchemaType >> =
    true extends {
        [K in keyof T]: IsVersionGreaterThan1 < T[K]['version'] > ;
    } [keyof T] ?
    true :
    false;

type MigrationPathsForSchema < T extends SchemaType > = T['version'] extends 1 ?
    {} // No migrations needed for version 1
    :
    {
        [K in EnumerateFrom2To < T['version'] > ]: (doc: Doc < T > ) => Doc < T > ;
    };
type MigrationPathsForSchemas < T extends SchemaTypeRecord > = {
    [K in keyof T]: MigrationPathsForSchema < T[K] > ;
};

type MigrationsParameter < T extends SchemaTypeRecord > =
    AnyVersionGreaterThan1 < T > extends true ?
    {
        migrations: MigrationPathsForSchemas < T >
    } :
    {
        migrations ? : never
    };
"#;

#[derive(Clone)]
pub struct MigrationPlugin {
    pub(crate) base: BasePlugin,
}

impl MigrationPlugin {
    pub fn new() -> Result<MigrationPlugin, JsValue> {
        let base = BasePlugin::new()?;
        let plugin = MigrationPlugin {
            base,
        };
        let plugin_clone1 = plugin.clone();
        let recover_hook = Closure::wrap(Box::new(move |schema, content| {
            plugin_clone1.recover_hook(schema, content)
        }) as Box<dyn Fn(JsValue, JsValue) -> Result<JsValue, JsValue>>);

        let mut plugin = plugin;
        plugin.base.doc_recover_hook = recover_hook.into_js_value();
        Ok(plugin)
    }

    pub(crate)fn recover_hook(&self, schema_js: JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        Ok(content)
    }


}
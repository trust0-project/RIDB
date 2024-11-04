use std::cell::Ref;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::plugin::BasePlugin;
use crate::schema::Schema;
use js_sys::{JsString, Reflect};
use chacha20poly1305::{
    aead::{KeyInit},
};
use web_sys::console::log_1;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type EnumerateUpTo<
    N extends number,
    Acc extends number[] = []
> = Acc['length'] extends N ?
    Acc[number]:
    EnumerateUpTo<N, [...Acc, Acc['length']]> ;

export type EnumerateFrom1To<
    N extends number
> = Exclude<EnumerateUpTo<N>,0> | (N extends 0 ? never : N);

export type IsVersionGreaterThan0<
    V extends number
> = V extends 0 ? false : true;

export type AnyVersionGreaterThan1<
    T extends Record<string, SchemaType>
> = true extends {
    [K in keyof T]: IsVersionGreaterThan0<T[K]['version']>;
} [keyof T] ? true : false;

export type MigrationFunction<T extends SchemaType> = (doc: Doc <T> ) => Doc <T>

export type MigrationPathsForSchema<
    T extends SchemaType
> = T['version'] extends 0 ? {}: // No migrations needed for version 1
    {
        [K in EnumerateFrom1To < T['version'] > ]: MigrationFunction<T> ;
    };

export type MigrationPathsForSchemas<
    T extends SchemaTypeRecord
> = {
    [K in keyof T]: MigrationPathsForSchema<T[K]>;
};

export type MigrationsParameter<
    T extends SchemaTypeRecord
> = AnyVersionGreaterThan1<T> extends true ?
    {
        migrations: MigrationPathsForSchemas<T>
    }:
    {
        migrations?: never
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
        let create_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            plugin_clone1.create_hook(schema, migration, content)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, JsValue>>);

        let plugin_clone2 = plugin.clone();
        let recover_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            plugin_clone2.recover_hook(schema, migration, content)
        }) as Box<dyn Fn(JsValue,JsValue,JsValue) -> Result<JsValue, JsValue>>);

        let mut plugin = plugin;
        plugin.base.doc_create_hook = create_hook.into_js_value();
        plugin.base.doc_recover_hook = recover_hook.into_js_value();
        Ok(plugin)
    }

    pub(crate) fn create_hook(
        &self,
        schema_js: JsValue,
        migration_js: JsValue,
        content: JsValue,
    ) -> Result<JsValue, JsValue> {
        let doc_version_key = JsValue::from("__version");
        let schema = Schema::create(schema_js)?;
        let version = schema.version;
        let doc_version = Reflect::get(&content, &doc_version_key)
            .map_err(|e| JsValue::from(format!("Error")))?;

        if doc_version.is_undefined() {
            Reflect::set(&content, &doc_version_key, &JsValue::from(version.to_owned()))?;
        }
        Ok(content)
    }

    pub(crate) fn recover_hook(
        &self,
        schema_js: JsValue,
        migration_js: JsValue,
        mut content: JsValue
    ) -> Result<JsValue, JsValue> {

        let doc_version_key = JsValue::from("__version");
        let schema = Schema::create(schema_js.clone())?;

        //Ensure that we have the version set correctly
        content = self.create_hook(schema_js.clone(), migration_js.clone(), content)?;
        let version = schema.version;

        let doc_version_js = Reflect::get(
            &content,
            &doc_version_key
        ).map_err(|e| JsValue::from(format!("Error")))?;

        let doc_version = if doc_version_js.is_undefined() {
            version
        } else {
            doc_version_js.as_f64()
                .ok_or_else(|| JsValue::from("__version should be a number"))? as i32
        };

        if doc_version < version {
            // Iterate through each version that needs migration
            for current_version in doc_version..version {
                // Get the next version's migration function
                let next_version = current_version+1;

                if migration_js.is_undefined() {
                    return Err(JsValue::from("Migration Object is undefined".to_string()))
                }

                let function = Reflect::get(
                    &migration_js, &JsValue::from(next_version)
                ).map_err(|e| JsValue::from(format!("Error")))?;

                if function.is_undefined() {
                    return Err(JsValue::from(format!("Migrating function {} to schema version not found", next_version)))
                }

                let upgraded = Reflect::apply(
                    &function.unchecked_into(),
                    &JsValue::NULL,
                    &js_sys::Array::of1(&content)
                )?;

                Reflect::set(&upgraded, &doc_version_key, &JsValue::from(next_version))?;

                content = upgraded;
            }
        }

        Ok(content)
    }

}
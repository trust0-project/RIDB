use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::plugin::BasePlugin;
use crate::schema::Schema;
use js_sys::Reflect;
use crate::error::RIDBError;

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
    pub fn new() -> Result<MigrationPlugin, RIDBError> {
        let base = BasePlugin::new("Migration".to_string())?;
        let plugin = MigrationPlugin {
            base,
        };

        let plugin_clone1 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            plugin_clone1.create_hook(schema, migration, content)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, RIDBError>>);

        let plugin_clone2 = plugin.clone();
        let recover_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            plugin_clone2.recover_hook(schema, migration, content)
        }) as Box<dyn Fn(JsValue,JsValue,JsValue) -> Result<JsValue, RIDBError>>);

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
    ) -> Result<JsValue, RIDBError> {
        // Handle both single object and array of objects
        if content.is_array() {
            let array = js_sys::Array::from(&content);
            let processed_array = js_sys::Array::new();
            
            for i in 0..array.length() {
                let item = array.get(i);
                match self.create_hook_single_document(schema_js.clone(), migration_js.clone(), item) {
                    Ok(processed_item) => {
                        processed_array.push(&processed_item);
                    },
                    Err(e) => return Err(e),
                }
            }
            
            Ok(processed_array.into())
        } else {
            // Handle single document
            self.create_hook_single_document(schema_js, migration_js, content)
        }
    }

    fn create_hook_single_document(
        &self,
        schema_js: JsValue,
        _migration_js: JsValue,
        content: JsValue,
    ) -> Result<JsValue, RIDBError> {
        let doc_version_key = JsValue::from("__version");
        let schema = Schema::create(schema_js)?;
        let version = schema.version;
        let doc_version = Reflect::get(&content, &doc_version_key)?;
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
    ) -> Result<JsValue, RIDBError> {
        let doc_version_key = JsValue::from("__version");
        let schema = Schema::create(schema_js.clone())?;
        //Ensure that we have the version set correctly
        content = self.create_hook(schema_js.clone(), migration_js.clone(), content)?;
        let version = schema.version;
        let doc_version_js = Reflect::get(
            &content,
            &doc_version_key
        ).map_err(|e| RIDBError::from(format!("Error getting the document version, err {:?}", e)))?;
        let doc_version = if doc_version_js.is_undefined() {
            version
        } else {
            doc_version_js.as_f64()
                .ok_or_else(|| RIDBError::from("__version should be a number"))? as i32
        };
        if doc_version < version {
            // Iterate through each version that needs migration
            for current_version in doc_version..version {
                // Get the next version's migration function
                let next_version = current_version+1;
                if migration_js.is_undefined() {
                    return Err(RIDBError::from("Migration Object is undefined".to_string()).into())
                }
                let function = Reflect::get(
                    &migration_js, &JsValue::from(next_version)
                ).map_err(|e| RIDBError::from(format!("Error recovering migration function for version {:?}", e)))?;
                if function.is_undefined() {
                    return Err(RIDBError::from(format!("Migrating function {} to schema version not found", next_version)).into())
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

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::{Object, JSON};
    use wasm_bindgen::__rt::IntoJsResult;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_basic_migration() {
        // Schema v1
        let schema_js = r#"{
            "version": 2,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "data": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        
        // Content in v1 format
        let content = JSON::parse(r#"{
            "id": "123",
            "data": "test",
            "__version":1
        }"#).unwrap();

        // Migration function that adds a new field
        let migrations = Object::new().into_js_result().unwrap();
        let migration_fn = js_sys::Function::new_with_args(
            "doc",
            "doc.newField = 'migrated'; return doc;"
        );

        Reflect::set(
            &migrations,
            &JsValue::from(2),
            &migration_fn
        ).unwrap();

        let plugin = MigrationPlugin::new().unwrap();
        
        // First create to set initial version
        let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
        // Then recover to trigger migration
        let migrated = plugin.recover_hook(schema_value, migrations.clone(), content).unwrap();
        assert_eq!(
            Reflect::get(&migrated, &JsValue::from_str("newField"))
                .unwrap()
                .as_string()
                .unwrap(),
            "migrated"
        );
        assert_eq!(
            Reflect::get(&migrated, &JsValue::from_str("__version"))
                .unwrap()
                .as_f64()
                .unwrap(),
            2.0
        );
    }

    #[wasm_bindgen_test]
    fn test_multiple_version_migrations() {
        let schema_js = r#"{
            "version": 3,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "data": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        
        let content = JSON::parse(r#"{
            "id": "123",
            "data": "test",
            "__version":1
        }"#).unwrap();

        // Migration function that adds a new field
        let migrations = Object::new().into_js_result().unwrap();
        let migration2_fn = js_sys::Function::new_with_args(
            "doc",
            "doc.v2field = 'v2'; return doc;"
        );
        let migration3_fn = js_sys::Function::new_with_args(
            "doc",
            "doc.v3field = 'v3'; return doc;"
        );

        Reflect::set(
            &migrations,
            &JsValue::from(2),
            &migration2_fn
        ).unwrap();

        Reflect::set(
            &migrations,
            &JsValue::from(3),
            &migration3_fn
        ).unwrap();

        let plugin = MigrationPlugin::new().unwrap();
        let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
        let migrated = plugin.recover_hook(schema_value, migrations, content).unwrap();
        
        assert_eq!(
            Reflect::get(&migrated, &JsValue::from_str("v2field"))
                .unwrap()
                .as_string()
                .unwrap(),
            "v2"
        );
        assert_eq!(
            Reflect::get(&migrated, &JsValue::from_str("v3field"))
                .unwrap()
                .as_string()
                .unwrap(),
            "v3"
        );
        assert_eq!(
            Reflect::get(&migrated, &JsValue::from_str("__version"))
                .unwrap()
                .as_f64()
                .unwrap(),
            3.0
        );
    }

        #[wasm_bindgen_test]
        fn test_no_migration_needed() {
            let schema_js = r#"{
                "version": 2,
                "primaryKey": "id",
                "type": "object",
                "properties": {
                    "id": {"type": "string"}
                }
            }"#;
            let schema_value = JSON::parse(schema_js).unwrap();

            // Content already at latest version
            let content = JSON::parse(r#"{
                "id": "123",
                "__version": 2
            }"#).unwrap();

            let migrations = Object::new().into_js_result().unwrap();
            let migration2_fn = js_sys::Function::new_with_args(
                "doc",
                "doc.shouldNotBeCalled = true; return doc;"
            );

            Reflect::set(
                &migrations,
                &JsValue::from(2),
                &migration2_fn
            ).unwrap();
            let plugin = MigrationPlugin::new().unwrap();
            let result = plugin.recover_hook(schema_value, migrations, content).unwrap();

            assert!(Reflect::get(&result, &JsValue::from_str("shouldNotBeCalled")).unwrap().is_undefined());
        }

          #[wasm_bindgen_test]
          fn test_missing_migration_function() {
              let schema_js = r#"{
                  "version": 2,
                  "primaryKey": "id",
                  "type": "object",
                  "properties": {
                      "id": {"type": "string"}
                  }
              }"#;
              let schema_value = JSON::parse(schema_js).unwrap();

              let content = JSON::parse(r#"{
                  "id": "123",
                  "__version": 1
              }"#).unwrap();

              // Missing migration function for version 2
              let migrations = JSON::parse("{}").unwrap();

              let plugin = MigrationPlugin::new().unwrap();
              let result = plugin.recover_hook(schema_value, migrations, content);
              assert!(result.is_err());
          }

          #[wasm_bindgen_test]
          fn test_create_hook_version_handling() {
              let schema_js = r#"{
                  "version": 1,
                  "primaryKey": "id",
                  "type": "object",
                  "properties": {
                      "id": {"type": "string"}
                  }
              }"#;
              let schema_value = JSON::parse(schema_js).unwrap();

              // Content without version
              let content = JSON::parse(r#"{"id": "123"}"#).unwrap();

              let plugin = MigrationPlugin::new().unwrap();
              let result = plugin.create_hook(schema_value, JsValue::NULL, content).unwrap();

              assert_eq!(
                  Reflect::get(&result, &JsValue::from_str("__version"))
                      .unwrap()
                      .as_f64()
                      .unwrap(),
                  1.0
              );
          }

             #[wasm_bindgen_test]
             fn test_undefined_migrations_object() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "__version": 1
                 }"#).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let result = plugin.recover_hook(schema_value, JsValue::UNDEFINED, content);
                 assert!(result.is_err());
             }

             #[wasm_bindgen_test]
             fn test_complex_data_type_migration() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"},
                         "nested": {"type": "object", "properties": {"newp":{"type":"string"}}},
                         "array": {"type": "array", "items": {"type": "number"}}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "nested": {"old": "data"},
                     "array": [1, 2, 3],
                     "__version":1
                 }"#).unwrap();

                let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.nested.newp = 'value'; doc.array.push(4); return doc;"
                );

                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
                 let migrated = plugin.recover_hook(schema_value, migrations, content).unwrap();

                 let nested = Reflect::get(&migrated, &JsValue::from_str("nested")).unwrap();
                 assert_eq!(
                     Reflect::get(&nested, &JsValue::from_str("newp"))
                         .unwrap()
                         .as_string()
                         .unwrap(),
                     "value"
                 );

                 let array = Reflect::get(&migrated, &JsValue::from_str("array")).unwrap();
                 assert_eq!(js_sys::Array::from(&array).length(), 4);
             }

             #[wasm_bindgen_test]
             fn test_migration_with_field_removal() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"},
                         "newField": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "oldField": "should be removed",
                     "__version":1
                 }"#).unwrap();


                 let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.newField = 'new value'; delete doc.oldField; return doc;"
                );

                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
                 let migrated = plugin.recover_hook(schema_value, migrations, content).unwrap();

                 assert!(Reflect::get(&migrated, &JsValue::from_str("oldField")).unwrap().is_undefined());
                 assert_eq!(
                     Reflect::get(&migrated, &JsValue::from_str("newField"))
                         .unwrap()
                         .as_string()
                         .unwrap(),
                     "new value"
                 );
             }

             #[wasm_bindgen_test]
             fn test_migration_error_handling() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "__version": 1
                 }"#).unwrap();

                 let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "throw new Error(\"Migration error\");"
                );

                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();




                 let plugin = MigrationPlugin::new().unwrap();
                 let result = plugin.recover_hook(schema_value, migrations, content);
                 assert!(result.is_err());
             }

             #[wasm_bindgen_test]
             fn test_migration_with_type_conversion() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"},
                         "value": {"type": "number"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "value": "42",
                     "__version":1
                 }"#).unwrap();

                let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.value = parseInt(doc.value); return doc;"
                );

                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
                 let migrated = plugin.recover_hook(schema_value, migrations, content).unwrap();

                 assert_eq!(
                     Reflect::get(&migrated, &JsValue::from_str("value"))
                         .unwrap()
                         .as_f64()
                         .unwrap(),
                     42.0
                 );
             }

             #[wasm_bindgen_test]
             fn test_migration_version_jump() {
                 let schema_js = r#"{
                     "version": 4,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123",
                     "__version": 1
                 }"#).unwrap();


                let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.v2 = true; return doc;"
                );
                    let migration3_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.v3 = true; return doc;"
                );
                    let migration4_fn = js_sys::Function::new_with_args(
                    "doc",
                    "doc.v4 = true; return doc;"
                );

                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();
                Reflect::set(
                    &migrations,
                    &JsValue::from(3),
                    &migration3_fn
                ).unwrap();
                Reflect::set(
                    &migrations,
                    &JsValue::from(4),
                    &migration4_fn
                ).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let content = plugin.create_hook(schema_value.clone(), migrations.clone(), content).unwrap();
                 let migrated = plugin.recover_hook(schema_value, migrations, content).unwrap();

                 // Verify all migrations were applied in order
                 assert!(Reflect::get(&migrated, &JsValue::from_str("v2")).unwrap().as_bool().unwrap());
                 assert!(Reflect::get(&migrated, &JsValue::from_str("v3")).unwrap().as_bool().unwrap());
                 assert!(Reflect::get(&migrated, &JsValue::from_str("v4")).unwrap().as_bool().unwrap());
                 assert_eq!(
                     Reflect::get(&migrated, &JsValue::from_str("__version"))
                         .unwrap()
                         .as_f64()
                         .unwrap(),
                     4.0
                 );
             }

             #[wasm_bindgen_test]
             fn test_create_hook_automaticallly_existing_version() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 let content = JSON::parse(r#"{
                     "id": "123"
                 }"#).unwrap();

                 let plugin = MigrationPlugin::new().unwrap();
                 let result = plugin.create_hook(schema_value, JsValue::NULL, content).unwrap();

                 assert_eq!(
                     Reflect::get(&result, &JsValue::from_str("__version"))
                         .unwrap()
                         .as_f64()
                         .unwrap(),
                     2.0
                 );
             }

             #[wasm_bindgen_test]
             fn test_invalid_version_format() {
                 let schema_js = r#"{
                     "version": 2,
                     "primaryKey": "id",
                     "type": "object",
                     "properties": {
                         "id": {"type": "string"}
                     }
                 }"#;
                 let schema_value = JSON::parse(schema_js).unwrap();

                 // Invalid version format (string instead of number)
                 let content = JSON::parse(r#"{
                     "id": "123",
                     "__version": "1"
                 }"#).unwrap();

    let migrations = Object::new().into_js_result().unwrap();
                let migration2_fn = js_sys::Function::new_with_args(
                    "doc",
                    "return doc;"
                );
                Reflect::set(
                    &migrations,
                    &JsValue::from(2),
                    &migration2_fn
                ).unwrap();


                 let plugin = MigrationPlugin::new().unwrap();
                 let result = plugin.recover_hook(schema_value, migrations, content);
                 assert!(result.is_err());
             }


}
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::plugin::BasePlugin;
use crate::schema::Schema;
use js_sys::{Object, Reflect};
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use pbkdf2::pbkdf2_hmac;
use sha3::Sha3_256;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use getrandom::getrandom;

#[derive(Clone)]
pub struct EncryptionPlugin {
    pub(crate) base: BasePlugin,
    pub(crate) password: String,
}

fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], JsValue> {
    if password.is_empty() {
        return Err(JsValue::from("Password cannot be empty"));
    }
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha3_256>(password.as_bytes(), salt, 5000, &mut key);
    Ok(key)
}


impl EncryptionPlugin {

    pub(crate) fn new(password: String) -> Result<EncryptionPlugin, JsValue> {
        let base = BasePlugin::new("Encryption".to_string())?;
        let plugin = EncryptionPlugin {
            base,
            password,
        };
        let plugin_clone1 = plugin.clone();
        let plugin_clone2 = plugin.clone();
        let create_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            // Add logging for debugging
            let result = plugin_clone1.clone().encrypt(schema, migration, content);
            result
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, JsValue>>);

        let recover_hook = Closure::wrap(Box::new(move |schema, migration, content| {
            // Add logging for debugging
            let result = plugin_clone2.clone().decrypt(schema, migration, content);
            result
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, JsValue>>);

        let mut plugin = plugin;
        plugin.base.doc_create_hook = create_hook.into_js_value();
        plugin.base.doc_recover_hook = recover_hook.into_js_value();
        Ok(plugin)
    }

    pub(crate) fn encrypt(&self, schema_js: JsValue, migration: JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        // Handle both single object and array of objects
        if content.is_array() {
            let array = js_sys::Array::from(&content);
            let encrypted_array = js_sys::Array::new();
            
            for i in 0..array.length() {
                let item = array.get(i);
                match self.encrypt_single_document(schema_js.clone(), migration.clone(), item) {
                    Ok(encrypted_item) => {
                        encrypted_array.push(&encrypted_item);
                    },
                    Err(e) => return Err(e),
                }
            }
            
            Ok(encrypted_array.into())
        } else {
            // Handle single document
            self.encrypt_single_document(schema_js, migration, content)
        }
    }

    fn encrypt_single_document(&self, schema_js: JsValue, _migration: JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        // Add validation for input parameters
        if schema_js.is_undefined() || schema_js.is_null() {
            return Err(JsValue::from("Schema cannot be null or undefined"));
        }
        if content.is_undefined() || content.is_null() {
            return Err(JsValue::from("Content cannot be null or undefined"));
        }

        let schema = Schema::create(schema_js)?;
        let encrypted = schema.encrypted.unwrap_or_default();
        
        // Validate content is an object
        if !content.is_object() {
            return Err(JsValue::from("Content must be an object"));
        }
        
        // Early return if no fields to encrypt
        if encrypted.is_empty() {
            return Ok(content);
        }
        // Create a mutable copy of content
        let content_obj = Object::from(content);
        let encrypted_obj = Object::new();
        let mut has_encrypted_fields = false;

        for field in encrypted {
            if schema.primary_key == field {
                return Err(JsValue::from("primary key must not be encrypted"));
            }
            if !schema.properties.contains_key(&field) {
                return Err(JsValue::from("encrypted field does not exist in the model"));
            }
            
            let property_key = JsValue::from(&field);
            if let Ok(property_value) = Reflect::get(&content_obj, &property_key) {
                if !property_value.is_undefined() && !property_value.is_null() {
                    has_encrypted_fields = true;
                    Reflect::set(&encrypted_obj, &property_key, &property_value)?;
                    Reflect::delete_property(&content_obj, &property_key)?;
                }
            }
        }

        // Only perform encryption if there are actual fields to encrypt
        if has_encrypted_fields {
            let serialized = js_sys::JSON::stringify(&encrypted_obj)
                .map_err(|_| JsValue::from("Failed to stringify encrypted data"))?;
            let serialized_bytes = serialized.as_string()
                .ok_or_else(|| JsValue::from("Failed to convert serialized data to string"))?
                .as_bytes()
                .to_vec();

            // Generate random salt and nonce
            let mut salt = [0u8; 16];
            getrandom(&mut salt).map_err(|e| JsValue::from(e.to_string()))?;
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

            // Derive key using PBKDF2
            let key = derive_key(&self.password, &salt)?;
            let cipher = Aes256Gcm::new_from_slice(&key)
                .map_err(|_| JsValue::from("Failed to create cipher"))?;

            let encrypted_data = cipher
                .encrypt(&nonce, serialized_bytes.as_ref())
                .map_err(|_| JsValue::from("Encryption failed"))?;

            // Combine salt, nonce, and ciphertext
            let mut combined = salt.to_vec();
            combined.extend_from_slice(&nonce);
            combined.extend(encrypted_data);

            let encoded = BASE64.encode(combined);

            Reflect::set(
                &content_obj,
                &JsValue::from_str("__encrypted"),
                &JsValue::from_str(&encoded),
            )?;
        }

        Ok(JsValue::from(content_obj.clone()))
    }
    
    pub(crate) fn decrypt(&self, schema_js: JsValue, migration: JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        
        // Add validation for input parameters
        if schema_js.is_undefined() || schema_js.is_null() {
            return Err(JsValue::from("Schema cannot be null or undefined"));
        }
        if content.is_undefined() || content.is_null() {
            return Err(JsValue::from("Content cannot be null or undefined"));
        }

        // Handle both single object and array of objects
        if content.is_array() {
            let array = js_sys::Array::from(&content);
            let decrypted_array = js_sys::Array::new();
            
            for i in 0..array.length() {
                let item = array.get(i);
                    match self.decrypt_single_document(schema_js.clone(), migration.clone(), item) {
                        Ok(decrypted_item) => {
                            decrypted_array.push(&decrypted_item);
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
            }
            
            Ok(decrypted_array.into())
        } else {
            // Handle single document
            self.decrypt_single_document(schema_js, migration, content)
        }
    }

    fn decrypt_single_document(&self, schema_js: JsValue, _migration: JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        // Validate content is an object
        if !content.is_object() {
            return Err(JsValue::from("Content must be an object"));
        }

        let content_obj = Object::from(content);
        
        // Safe get of encrypted data
        let encrypted_data = match Reflect::get(&content_obj, &JsValue::from_str("__encrypted")) {
            Ok(data) => data,
            Err(_) => return Err(JsValue::from("Failed to read encrypted data")),
        };

        if encrypted_data.is_undefined() {
            return Ok(JsValue::from(content_obj));
        }

        let schema = Schema::create(schema_js)?;
        let encrypted = schema.encrypted.unwrap_or_default();

        // Validate we have fields to decrypt
        if encrypted.is_empty() {
            return Ok(JsValue::from(content_obj));
        }

        // Get the encrypted data string with better error handling
        let encrypted_str = encrypted_data
            .as_string()
            .ok_or_else(|| JsValue::from("Invalid encrypted data: expected string"))?;

        // Add minimum length check for base64 data
        if encrypted_str.is_empty() {
            return Err(JsValue::from("Encrypted data is empty"));
        }

        // Decode base64
        let decoded = BASE64
            .decode(encrypted_str)
            .map_err(|_| JsValue::from("Invalid base64 data"))?;

        if decoded.len() < 28 {
            return Err(JsValue::from("Invalid encrypted data length"));
        }

        // Split salt, nonce, and ciphertext
        let (salt, rest) = decoded.split_at(16);
        let (nonce_bytes, ciphertext) = rest.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Derive key using PBKDF2
        let key = derive_key(&self.password, salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| JsValue::from("Failed to create cipher"))?;

        // Decrypt the data
        let decrypted_data = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| JsValue::from("Decryption failed"))?;

        let decrypted_str = String::from_utf8(decrypted_data)
            .map_err(|_| JsValue::from("Invalid UTF-8 data"))?;

        // Parse the decrypted JSON string back into a JS object
        let encrypted_obj = js_sys::JSON::parse(&decrypted_str)
            .map_err(|_| JsValue::from("Failed to parse decrypted data"))?;

        // Remove the encrypted field
        Reflect::delete_property(&content_obj, &JsValue::from_str("__encrypted"))?;
        
        // Merge the decrypted fields back into the content
        for field in encrypted {
            let key = JsValue::from(field);
            if let Ok(value) = Reflect::get(&encrypted_obj, &key) {
                if !value.is_undefined() {
                    Reflect::set(&content_obj, &key, &value)?;
                }
            }
        }

        Ok(JsValue::from(content_obj))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::JSON;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_encryption_basic() {
        // Create a schema with encrypted fields
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["secret"],
            "properties": {
                "id": {"type": "string"},
                "secret": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        
        // Create test content
        let content_js = r#"{
            "id": "123",
            "secret": "sensitive data"
        }"#;
        let content = JSON::parse(content_js).unwrap();

        // Test encryption
        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let encrypted = plugin.encrypt(schema_value.clone(), JsValue::NULL, content.clone()).unwrap();
        
        // Verify encrypted field is removed and replaced with encrypted data
        assert!(Reflect::get(&encrypted, &JsValue::from_str("secret")).unwrap().is_undefined());
        assert!(!Reflect::get(&encrypted, &JsValue::from_str("__encrypted")).unwrap().is_undefined());

        // Test decryption
        let decrypted = plugin.decrypt(schema_value, JsValue::NULL, encrypted).unwrap();
        let secret = Reflect::get(&decrypted, &JsValue::from_str("secret")).unwrap();
        assert_eq!(secret.as_string().unwrap(), "sensitive data");
    }

    #[wasm_bindgen_test]
    fn test_encryption_primary_key_error() {
        // Try to encrypt primary key (should fail)
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["id"],
            "properties": {
                "id": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        let content = JSON::parse(r#"{"id": "123"}"#).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let result = plugin.encrypt(schema_value, JsValue::NULL, content);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_encryption_no_encrypted_fields() {
        // Test with no encrypted fields specified
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "name": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        let content = JSON::parse(r#"{"id": "123", "name": "test"}"#).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let result = plugin.encrypt(schema_value.clone(), JsValue::NULL, content.clone()).unwrap();
        
        // Content should remain unchanged
        assert_eq!(
            JSON::stringify(&result).unwrap(),
            JSON::stringify(&content).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_multiple_encrypted_fields() {
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["secret1", "secret2"],
            "properties": {
                "id": {"type": "string"},
                "secret1": {"type": "string"},
                "secret2": {"type": "number"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        
        let content_js = r#"{
            "id": "123",
            "secret1": "sensitive data",
            "secret2": 42
        }"#;
        let content = JSON::parse(content_js).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let encrypted = plugin.encrypt(schema_value.clone(), JsValue::NULL, content).unwrap();
        
        // Verify both fields are removed
        assert!(Reflect::get(&encrypted, &JsValue::from_str("secret1")).unwrap().is_undefined());
        assert!(Reflect::get(&encrypted, &JsValue::from_str("secret2")).unwrap().is_undefined());
        
        // Verify decryption restores both fields
        let decrypted = plugin.decrypt(schema_value, JsValue::NULL, encrypted).unwrap();
        assert_eq!(
            Reflect::get(&decrypted, &JsValue::from_str("secret1"))
                .unwrap()
                .as_string()
                .unwrap(),
            "sensitive data"
        );
        assert_eq!(
            Reflect::get(&decrypted, &JsValue::from_str("secret2"))
                .unwrap()
                .as_f64()
                .unwrap(),
            42.0
        );
    }

    #[wasm_bindgen_test]
    fn test_different_data_types() {
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["string_field", "number_field", "boolean_field", "object_field", "array_field"],
            "properties": {
                "id": {"type": "string"},
                "string_field": {"type": "string"},
                "number_field": {"type": "number"},
                "boolean_field": {"type": "boolean"},
                "object_field": {"type": "object", "properties": {"key":{"type":"string"}}},
                "array_field": {"type": "array", "items": [{"type": "number"}]}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        
        let content_js = r#"{
            "id": "123",
            "string_field": "test",
            "number_field": 42,
            "boolean_field": true,
            "object_field": {"key": "value"},
            "array_field": [1, 2, 3]
        }"#;
        let content = JSON::parse(content_js).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let encrypted = plugin.encrypt(schema_value.clone(), JsValue::NULL, content.clone()).unwrap();
        let decrypted = plugin.decrypt(schema_value, JsValue::NULL, encrypted).unwrap();

        // Verify all fields are correctly restored
        assert_eq!(
            JSON::stringify(&decrypted).unwrap(),
            JSON::stringify(&content.clone()).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_invalid_password_decryption() {
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["secret"],
            "properties": {
                "id": {"type": "string"},
                "secret": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        let content = JSON::parse(r#"{"id": "123", "secret": "test"}"#).unwrap();

        // Encrypt with one password
        let plugin1 = EncryptionPlugin::new("password1".to_string()).unwrap();
        let encrypted = plugin1.encrypt(schema_value.clone(), JsValue::NULL, content).unwrap();

        // Try to decrypt with different password
        let plugin2 = EncryptionPlugin::new("password2".to_string()).unwrap();
        let result = plugin2.decrypt(schema_value, JsValue::NULL, encrypted);
        assert!(result.is_err());
    }

    // #[wasm_bindgen_test]
    // fn test_corrupted_encrypted_data() {
    //     let schema_js = r#"{
    //         "version": 1,
    //         "primaryKey": "id",
    //         "type": "object",
    //         "encrypted": ["secret"],
    //         "properties": {
    //             "id": {"type": "string"},
    //             "secret": {"type": "string"}
    //         }
    //     }"#;
    //     let schema_value = JSON::parse(schema_js).unwrap();
        
    //     // Create content with corrupted encrypted data
    //     let content = JSON::parse(r#"{
    //         "id": "123",
    //         "encrypted": "not-valid-base64!"
    //     }"#).unwrap();

    //     let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
    //     let result = plugin.decrypt(schema_value, JsValue::undefined(), content);
    //     assert!(result.is_err());
    // }

    #[wasm_bindgen_test]
    fn test_empty_encrypted_fields() {
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": [],
            "properties": {
                "id": {"type": "string"},
                "data": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        let content = JSON::parse(r#"{"id": "123", "data": "test"}"#).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let result = plugin.encrypt(schema_value.clone(), JsValue::NULL, content.clone()).unwrap();
        
        // Content should remain unchanged
        assert_eq!(
            JSON::stringify(&result).unwrap(),
            JSON::stringify(&content).unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_nonexistent_field_encryption() {
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["nonexistent"],
            "properties": {
                "id": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();
        let content = JSON::parse(r#"{"id": "123"}"#).unwrap();

        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let result = plugin.encrypt(schema_value, JsValue::NULL, content);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_aes_gcm_encryption() {
        // Create a schema with encrypted fields
        let schema_js = r#"{
            "version": 1,
            "primaryKey": "id",
            "type": "object",
            "encrypted": ["secret"],
            "properties": {
                "id": {"type": "string"},
                "secret": {"type": "string"}
            }
        }"#;
        let schema_value = JSON::parse(schema_js).unwrap();

        // Create test content
        let content_js = r#"{
            "id": "123",
            "secret": "sensitive data"
        }"#;
        let content = JSON::parse(content_js).unwrap();

        // Test encryption
        let plugin = EncryptionPlugin::new("test_password".to_string()).unwrap();
        let encrypted = plugin.encrypt(schema_value.clone(), JsValue::NULL, content.clone()).unwrap();

        // Verify encrypted field is removed and replaced with encrypted data
        assert!(Reflect::get(&encrypted, &JsValue::from_str("secret")).unwrap().is_undefined());
        assert!(!Reflect::get(&encrypted, &JsValue::from_str("__encrypted")).unwrap().is_undefined());

        // Test decryption
        let decrypted = plugin.decrypt(schema_value, JsValue::NULL, encrypted).unwrap();
        let secret = Reflect::get(&decrypted, &JsValue::from_str("secret")).unwrap();
        assert_eq!(secret.as_string().unwrap(), "sensitive data");
    }
}
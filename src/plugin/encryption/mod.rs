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

#[derive(Clone)]
pub struct EncryptionPlugin {
    pub(crate) base: BasePlugin,
    pub(crate) password: String,
}

fn derive_key(password: &str) -> [u8; 32] {
    // Note: In production, use a proper key derivation function like Argon2 or PBKDF2
    let mut key = [0u8; 32];
    let pass_bytes = password.as_bytes();
    for (i, &byte) in pass_bytes.iter().enumerate() {
        key[i % 32] ^= byte;
    }
    key
}


impl EncryptionPlugin {
    pub fn new(password: String) -> Result<EncryptionPlugin, JsValue> {
        let base = BasePlugin::new()?;
        let plugin = EncryptionPlugin {
            base,
            password,
        };
        
        let plugin_clone1 = plugin.clone();
        let plugin_clone2 = plugin.clone();
        
        let create_hook = Closure::wrap(Box::new(move |schema,migration, content| {
            plugin_clone1.encrypt(schema,migration,  content)
        }) as Box<dyn Fn(JsValue, JsValue, JsValue) -> Result<JsValue, JsValue>>);

        let recover_hook = Closure::wrap(Box::new(move |schema,migration, content| {
            plugin_clone2.decrypt(schema, migration, content)
        }) as Box<dyn Fn(JsValue,JsValue,  JsValue) -> Result<JsValue, JsValue>>);

        let mut plugin = plugin;
        plugin.base.doc_create_hook = create_hook.into_js_value();
        plugin.base.doc_recover_hook = recover_hook.into_js_value();

        Ok(plugin)
    }

    pub(crate) fn encrypt(&self, schema_js: JsValue, migration:JsValue, content: JsValue) -> Result<JsValue, JsValue> {
        let schema = Schema::create(schema_js)?;
        let encrypted = schema.encrypted.unwrap_or(Vec::new());
        
        let encrypted_obj = Object::new();
        for field in encrypted {
            if schema.primary_key == field {
                return Err(JsValue::from("primary key must not be encrypted"))
            }
            if !schema.properties.contains_key(&field) {
                return Err(JsValue::from("encrypted field does not exist in the model"))
            }
            let property_key = JsValue::from(field);
            let property_value = Reflect::get(&content, &property_key)?;
            Reflect::set(&encrypted_obj, &property_key, &property_value)?;
            let content_obj = Object::from(content.clone());
            Reflect::delete_property(
                &content_obj, &property_key
            )?;
        }
    
        if Object::keys(&encrypted_obj).length() > 0 {
            let serialized = js_sys::JSON::stringify(&encrypted_obj)
                .map_err(|_| JsValue::from("Failed to stringify encrypted data"))?;
            let serialized_str = serialized.as_string()
                .ok_or_else(|| JsValue::from("Failed to get string from JsValue"))?;
            let serialized_bytes = serialized_str.as_bytes();
    
            // Generate a random 12-byte nonce
            let nonce = rand::random::<[u8; 12]>();
            let nonce = Nonce::from_slice(&nonce);
    
            // Create cipher
            let key = derive_key(&self.password);
            let cipher = ChaCha20Poly1305::new_from_slice(&key)
                .map_err(|_| JsValue::from("Failed to create cipher"))?;
    
            // Encrypt the data
            let encrypted_data = cipher
                .encrypt(nonce, serialized_bytes)
                .map_err(|_| JsValue::from("Encryption failed"))?;
    
            // Combine nonce and encrypted data and encode as base64
            let mut combined = nonce.to_vec();
            combined.extend(encrypted_data);
            let encoded = BASE64.encode(combined);
    
            Reflect::set(
                &content,
                &JsValue::from_str("encrypted"),
                &JsValue::from_str(&encoded),
            )?;
        }
    
        Ok(content)
    }
    
    pub(crate)fn decrypt(&self, schema_js: JsValue,migration:JsValue,  content: JsValue) -> Result<JsValue, JsValue> {
        let encrypted_data = Reflect::get(&content, &JsValue::from_str("encrypted"))?;
        if encrypted_data.is_undefined() {
            return Ok(content);
        }
    
        let schema = Schema::create(schema_js)?;
        let encrypted = schema.encrypted.unwrap_or(Vec::new());
    
        // Get the encrypted data string
        let encrypted_str = encrypted_data
            .as_string()
            .ok_or_else(|| JsValue::from("Invalid encrypted data"))?;
    
        // Decode base64
        let decoded = BASE64
            .decode(encrypted_str)
            .map_err(|_| JsValue::from("Invalid base64 data"))?;
    
        if decoded.len() < 12 {
            return Err(JsValue::from("Invalid encrypted data length"));
        }
    
        // Split nonce and ciphertext
        let (nonce, ciphertext) = decoded.split_at(12);
        let nonce = Nonce::from_slice(nonce);
    
        // Create cipher
        let key = derive_key(self.password.as_str());
        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|_| JsValue::from("Failed to create cipher"))?;
    
        // Decrypt the data
        let decrypted_data = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| JsValue::from("Decryption failed"))?;
    
        let decrypted_str = String::from_utf8(decrypted_data)
            .map_err(|_| JsValue::from("Invalid UTF-8 data"))?;
    
        // Parse the decrypted JSON string back into a JS object
        let encrypted_obj = js_sys::JSON::parse(&decrypted_str)
            .map_err(|_| JsValue::from("Failed to parse encrypted data"))?;
    
        // Create a new object from the content
        let decrypted_content = Object::from(content.clone());
        
        // Remove the encrypted field
        Reflect::delete_property(&decrypted_content, &JsValue::from_str("encrypted"))?;
        
        // Merge the decrypted fields back into the content
        for field in encrypted {
            let key = JsValue::from(field);
            if let Ok(value) = Reflect::get(&encrypted_obj, &key) {
                Reflect::set(&decrypted_content, &key, &value)?;
            }
        }

        Ok(decrypted_content.into())
    }
}
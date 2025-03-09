use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::utils::extract_property;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Errors {
    Error,
    HookError,
    QueryError,
    SerializationError,
    ValidationError,
    AuthenticationError,
}

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RIDBError {
    pub(crate) err_type: String,
    pub(crate) message: String,
    pub(crate) code: u32
}

#[wasm_bindgen]
impl RIDBError {

    #[wasm_bindgen(constructor)]
    pub fn new(
        err_type: String,
        message: String,
        code: u32
    ) -> RIDBError {
        RIDBError {
            err_type,
            message,
            code
        }
    }

    #[wasm_bindgen(getter, js_name=type)]
    pub fn get_type(&self) -> String {
        self.err_type.to_string()
    }

    #[wasm_bindgen(getter, js_name=code)]
    pub fn get_code(&self) -> JsValue {
        JsValue::from(
            self.code
        )
    }

    #[wasm_bindgen(getter, js_name=message)]
    pub fn get_message(&self) -> String {
        self.message.to_string()
    }

    #[wasm_bindgen(js_name=from)]
    pub fn get_from(err: JsValue) -> RIDBError {
        RIDBError::from(err)
    }

    fn create_error(message: &str, code: u32, err_type: Errors) -> RIDBError {
        let message_type = match err_type {
            Errors::QueryError => "Query Error".to_string(),
            Errors::AuthenticationError => "Authentication Error".to_string(),
            Errors::SerializationError => "Serialization Error".to_string(),
            Errors::ValidationError => "Validation Error".to_string(),
            Errors::HookError => "Hook Error".to_string(),
            _ => "Error".to_string(),
        };
        let complete_message=  format!("{}: {}", message_type, message);
        RIDBError {
            err_type: format!("{:?}", err_type),
            code,
            message: complete_message.as_str().to_string()
        }
    }

    #[wasm_bindgen]
    pub fn error(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::Error)
    }

    #[wasm_bindgen]
    pub fn query(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::QueryError)
    }

    #[wasm_bindgen]
    pub fn authentication(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::AuthenticationError)
    }

    #[wasm_bindgen]
    pub fn serialisation(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::SerializationError)
    }

    #[wasm_bindgen]
    pub fn validation(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::ValidationError)
    }

    #[wasm_bindgen]
    pub fn hook(err: &str, code: u32) -> RIDBError {
        RIDBError::create_error(err, code, Errors::HookError)
    }
}

impl From<serde_wasm_bindgen::Error> for RIDBError {
    fn from(error: serde_wasm_bindgen::Error) -> RIDBError {
        RIDBError::serialisation(
            error.to_string().replace("Error: ", "").as_str(),
            0
        )
    }
}

impl From<&String> for RIDBError {
    fn from(error:&String) -> RIDBError {
        RIDBError::error(
            error.as_str(),
            0
        )
    }
}

impl From<&str> for RIDBError {
    fn from(error:&str) -> RIDBError {
        RIDBError::error(
            error,
            0
        )
    }
}

impl From<String> for RIDBError {
    fn from(error:String) -> RIDBError {
        RIDBError::error(
            error.as_str(),
            0
        )
    }
}

impl From<JsValue> for RIDBError {
    fn from(error: JsValue) -> RIDBError {
        let err_type_js = extract_property::<String>(&error, "type").unwrap_or("Error".to_string());
        let code = extract_property::<u32>(&error, "code").unwrap_or(0);
        let message = extract_property::<String>(&error, "message").expect("Invalid JS Error no message is available");
        RIDBError {
            err_type:err_type_js.to_string(),
            code,
            message: message.as_str().to_string()
        }
    }
}

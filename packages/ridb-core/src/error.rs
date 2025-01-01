use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use serde::de::value::Error;
use wasm_bindgen::prelude::*;
use crate::utils::extract_property;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Errors {
    Error,
    SerializationError,
    ValidationError
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RIDBError {
    pub code: Errors,
    pub message: String,
}

impl RIDBError {
    pub fn error(err: &str) -> RIDBError {
        RIDBError {
            code: Errors::Error,
            message:  format!("Serialization Error: {}", err)
        }
    }
    pub fn serialisation(err: &str) -> RIDBError {
        RIDBError {
            code: Errors::SerializationError,
            message: format!("Error: {}", err)
        }
    }
    pub fn validation(err: &str) -> RIDBError {
        RIDBError {
            code: Errors::ValidationError,
            message: format!("Validation Error: {}", err)
        }
    }
}

impl From<serde_wasm_bindgen::Error> for RIDBError {
    fn from(error: serde_wasm_bindgen::Error) -> RIDBError {
        RIDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<JsValue> for RIDBError {
    fn from(error: JsValue) -> RIDBError {
        let code = extract_property::<Errors>(&error, "code").unwrap_or(Errors::Error);
        let message = extract_property::<String>(&error, "message").expect("Invalid JS Error no message is available");
        RIDBError {
            code,
            message,
        }
    }
}

impl From<&str> for RIDBError {
    fn from(error:&str) -> RIDBError {
        RIDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<String> for RIDBError {
    fn from(error:String) -> RIDBError {
        RIDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<Error> for RIDBError {
    fn from(error:Error) -> RIDBError {
        RIDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<RIDBError> for JsValue {
    fn from(failure: RIDBError) -> Self {
        let error = js_sys::Error::new(&failure.message).into();
        Reflect::set(&error, &"code".into(), &serde_wasm_bindgen::to_value(&failure.code).unwrap()).unwrap();
        error
    }
}

impl From<Errors> for JsValue {
    fn from(failure: Errors) -> Self {
        serde_wasm_bindgen::to_value(&failure).unwrap()
    }
}

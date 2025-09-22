use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum TypeReponses {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Responses {
    Success(ResponseContent),
    Error(ResponseContent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContent {
    message: String,
    payload: Value,
}

impl Responses {
    pub fn new<T: Serialize>(type_response: TypeReponses, message: String, payload: T) -> Self {
        let content = ResponseContent {
            message,
            payload: serde_json::to_value(payload).unwrap(),
        };

        match type_response {
            TypeReponses::Success => Responses::Success(content),
            TypeReponses::Error => Responses::Error(content),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let str_json = serde_json::to_string(self).unwrap();
        str_json.into_bytes()
    }
}

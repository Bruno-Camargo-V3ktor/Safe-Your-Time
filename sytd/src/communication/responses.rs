use serde::{ Deserialize, Serialize };
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Responses {
    Success(ResponseContent),
    Error(ResponseContent),
    Panic(ResponseContent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContent {
    pub message: String,
    pub payload: Value,
}

impl Responses {
    pub fn success<T: Serialize>(message: String, payload: T) -> Self {
        let content = ResponseContent {
            message,
            payload: serde_json::to_value(payload).unwrap(),
        };

        Self::Success(content)
    }

    pub fn error<T: Serialize>(message: String, payload: T) -> Self {
        let content = ResponseContent {
            message,
            payload: serde_json::to_value(payload).unwrap(),
        };

        Self::Error(content)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let str_json = serde_json::to_string(self).unwrap();
        str_json.into_bytes()
    }
}

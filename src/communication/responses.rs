use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum Responses {
    Success(String),
    Error(String),
}

impl Responses {
    pub fn to_bytes(&self) -> Vec<u8> {
        let str_json = serde_json::to_string(self).unwrap();
        str_json.into_bytes()
    }
}

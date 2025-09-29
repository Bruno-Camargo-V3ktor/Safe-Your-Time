use std::collections::HashMap;
use super::{ AppConfig, TimeBlock };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub blocks: HashMap<String, TimeBlock>,
    pub config: AppConfig,
}

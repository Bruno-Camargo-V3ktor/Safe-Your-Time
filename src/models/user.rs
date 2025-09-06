use super::{AppConfig, TimeBlock};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub blocks: Vec<TimeBlock>,
    pub config: AppConfig,
}

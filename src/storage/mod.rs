use crate::models::{AppConfig, TimeBlock};

mod surrealdb_storage;
use serde::{Deserialize, Serialize};
pub use surrealdb_storage::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub blocks: Vec<TimeBlock>,
    pub config: AppConfig,
}

#[async_trait::async_trait]
pub trait Storage {
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn load(&self, username: String) -> anyhow::Result<Option<User>>;
    async fn create(&self, username: String) -> anyhow::Result<User>;
}

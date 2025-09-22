use std::sync::Arc;

use crate::models::User;

mod surrealdb_storage;
pub use surrealdb_storage::*;

pub type SharedStorage = Arc<dyn Storage + Send + Sync>;

#[async_trait::async_trait]
pub trait Storage {
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn load(&self, username: String) -> anyhow::Result<Option<User>>;
    async fn create(&self, username: String) -> anyhow::Result<User>;
}

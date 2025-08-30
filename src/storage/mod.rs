use crate::models::{AppConfig, TimeBlock};
use chrono::{DateTime, Duration, Local};

mod surrealdb_storage;
pub use surrealdb_storage::*;

pub type TimeBlockUpdate = (
    Option<Duration>,
    Option<String>,
    Option<DateTime<Local>>,
    Option<DateTime<Local>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
);

pub type AppConfigUpdate = (
    Option<Vec<String>>,
    Option<u16>,
    Option<u16>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<String>,
);

#[async_trait::async_trait]
pub trait Storage {
    async fn get_time_block(&self, user: String, name: String)
    -> anyhow::Result<Option<TimeBlock>>;

    async fn create_time_block(&self, user: String, time_block: TimeBlock) -> anyhow::Result<()>;

    async fn delete_time_block(&self, user: String, name: String) -> anyhow::Result<()>;

    async fn update_time_block(
        &self,
        user: String,
        update_args: TimeBlockUpdate,
    ) -> anyhow::Result<Option<TimeBlock>>;

    async fn get_all_time_block(&self, user: String) -> anyhow::Result<Vec<(String, TimeBlock)>>;

    async fn get_config(&self, user: String) -> anyhow::Result<AppConfig>;

    async fn update_config(
        &self,
        user: String,
        update_args: AppConfigUpdate,
    ) -> anyhow::Result<AppConfig>;
}

use crate::{
    models::{AppConfig, TimeBlock},
    storage::{AppConfigUpdate, Storage, TimeBlockUpdate},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealdb::{
    RecordId, Surreal,
    engine::local::{Db, RocksDb},
};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    config: AppConfig,
    blocks: Vec<TimeBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserWithId {
    pub id: RecordId,
    pub username: String,
    pub config: AppConfig,
    pub blocks: Vec<TimeBlock>,
}

#[derive(Clone, Debug)]
pub struct SurrealDbStorage {
    db: Surreal<Db>,
}

impl SurrealDbStorage {
    pub async fn new(
        path: impl Into<String>,
        ns: impl Into<String>,
        db: impl Into<String>,
    ) -> Self {
        let mut db_path = PathBuf::from(path.into());
        db_path.push("mem");
        std::fs::create_dir_all(&db_path).unwrap();

        let database = Surreal::new::<RocksDb>(db_path.to_str().unwrap())
            .await
            .unwrap();

        database.use_ns(ns).use_db(db).await.unwrap();

        let _ = database
            .query(
                r#"
            DEFINE TABLE IF NOT EXISTS users TYPE NORMAL SCHEMALESS PERMISSIONS NONE;

            DEFINE FIELD IF NOT EXISTS blocks ON users FLEXIBLE TYPE array<object> PERMISSIONS FULL;
            DEFINE FIELD IF NOT EXISTS blocks[*] ON users FLEXIBLE TYPE object PERMISSIONS FULL;
            DEFINE FIELD IF NOT EXISTS config ON users FLEXIBLE TYPE object PERMISSIONS FULL;
            DEFINE FIELD IF NOT EXISTS username ON users TYPE string PERMISSIONS FULL;

            DEFINE INDEX IF NOT EXISTS username_index ON users FIELDS username UNIQUE;
        "#,
            )
            .await;

        Self { db: database }
    }
}

#[async_trait::async_trait]
impl Storage for SurrealDbStorage {
    async fn get_time_block(
        &self,
        user: String,
        name: String,
    ) -> anyhow::Result<Option<TimeBlock>> {
        todo!()
    }

    async fn create_time_block(&self, user: String, time_block: TimeBlock) -> anyhow::Result<()> {
        let user = self.get_user_by_username(user.clone()).await?;
        let _ = self
            .db
            .query("UPDATE $id SET blocks += $block_time WHERE $name NOT IN blocks.name")
            .bind(("id", user.id))
            .bind(("block_time", time_block.clone()))
            .bind(("name", time_block.name))
            .await?;

        Ok(())
    }

    async fn delete_time_block(&self, user: String, name: String) -> anyhow::Result<()> {
        todo!()
    }

    async fn update_time_block(
        &self,
        user: String,
        update_args: TimeBlockUpdate,
    ) -> anyhow::Result<Option<TimeBlock>> {
        todo!()
    }

    async fn get_all_time_block(&self, user: String) -> anyhow::Result<Vec<(String, TimeBlock)>> {
        todo!()
    }

    async fn get_config(&self, user: String) -> anyhow::Result<AppConfig> {
        todo!()
    }

    async fn update_config(
        &self,
        user: String,
        update_args: AppConfigUpdate,
    ) -> anyhow::Result<AppConfig> {
        todo!()
    }
}

impl SurrealDbStorage {
    async fn create_user(&self, username: String) -> anyhow::Result<UserWithId> {
        let user = User {
            username,
            config: AppConfig::default_configs(),
            blocks: Vec::new(),
        };

        let user: UserWithId = self.db.create("users").content(user).await?.unwrap();
        Ok(user)
    }

    async fn get_user_by_username(&self, username: String) -> anyhow::Result<UserWithId> {
        let mut result = self
            .db
            .query("SELECT * FROM users WHERE username = $username LIMIT 1")
            .bind(("username", username))
            .await?;

        let user: Option<UserWithId> = result.take(0)?;
        let user = user.unwrap();

        Ok(user)
    }
}

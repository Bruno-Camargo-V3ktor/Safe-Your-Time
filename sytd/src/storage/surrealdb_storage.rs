use super::{SharedStorage, Storage};
use crate::models::{AppConfig, User};
use sha1::{Digest, Sha1};
use std::{path::PathBuf, sync::Arc};
use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};

#[derive(Clone, Debug)]
pub struct SurrealDbStorage {
    db: Surreal<Db>,
}

impl SurrealDbStorage {
    pub async fn new(
        path: impl Into<String>,
        ns: impl Into<String>,
        db: impl Into<String>,
    ) -> SharedStorage {
        let mut db_path = PathBuf::from(path.into());
        db_path.push("mem");
        std::fs::create_dir_all(&db_path).unwrap();

        let database = Surreal::new::<RocksDb>(db_path.to_str().unwrap())
            .await
            .unwrap();

        database.use_ns(ns).use_db(db).await.unwrap();

        let _ = database
            .query(r#"
                DEFINE TABLE IF NOT EXISTS users TYPE NORMAL SCHEMALESS PERMISSIONS NONE;

                DEFINE FIELD IF NOT EXISTS blocks ON users FLEXIBLE TYPE array<object> PERMISSIONS FULL;
                DEFINE FIELD IF NOT EXISTS blocks[*] ON users FLEXIBLE TYPE object PERMISSIONS FULL;
                DEFINE FIELD IF NOT EXISTS config ON users FLEXIBLE TYPE object PERMISSIONS FULL;
            "#,
            )
            .await;

        Arc::new(Self { db: database })
    }
}

#[async_trait::async_trait]
impl Storage for SurrealDbStorage {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        let user_id = hash_username(user.username.clone());
        let _: Option<User> = self
            .db
            .update(("users", &user_id))
            .content(user.clone())
            .await?;
        Ok(())
    }

    async fn load(&self, username: String) -> anyhow::Result<Option<User>> {
        let user_id = hash_username(username.clone());
        let user: Option<User> = self.db.select(("users", &user_id)).await?;
        Ok(user)
    }

    async fn create(&self, username: String) -> anyhow::Result<User> {
        let user_id = hash_username(username.clone());
        let user = User {
            username,
            blocks: vec![],
            config: AppConfig::default_configs(),
        };

        let created_user: User = self
            .db
            .create(("users", &user_id))
            .content(user)
            .await?
            .unwrap();
        Ok(created_user)
    }
}

pub fn hash_username(username: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&username.into_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}

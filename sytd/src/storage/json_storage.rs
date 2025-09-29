use std::{ collections::HashMap, fs::{ self, File }, path::PathBuf, sync::Arc };
use anyhow::Ok;
use sha1::{ Digest, Sha1 };
use crate::models::{ AppConfig, User };
use super::{ SharedStorage, Storage };

#[derive(Debug)]
pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub async fn new(path: impl Into<String>) -> SharedStorage {
        let mut full_path = PathBuf::from(path.into());
        full_path.push("mem.json");

        if !full_path.is_file() {
            File::create(&full_path).unwrap();
            let _ = fs::write(&full_path, "{}");
        }

        Arc::new(Self {
            path: full_path.to_str().unwrap().to_string(),
        })
    }

    async fn open_file(&self) -> anyhow::Result<HashMap<String, User>> {
        let file_str = fs::read_to_string(&self.path)?;
        let users: HashMap<String, User> = serde_json::from_str(&file_str)?;

        Ok(users)
    }

    async fn save_file(&self, users: &HashMap<String, User>) -> anyhow::Result<()> {
        let file_str = serde_json::to_string_pretty(&users)?;
        fs::write(&self.path, file_str)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Storage for JsonStorage {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        let mut users = self.open_file().await?;

        let user_id = hash_username(user.username.clone());
        users.insert(user_id, user.clone());

        self.save_file(&users).await?;
        Ok(())
    }

    async fn load(&self, username: String) -> anyhow::Result<Option<User>> {
        let users = self.open_file().await?;

        Ok(users.get(&username).cloned())
    }

    async fn create(&self, username: String) -> anyhow::Result<User> {
        let mut users = self.open_file().await?;

        let user_id = hash_username(username.clone());
        let new_user = User {
            blocks: vec![],
            username,
            config: AppConfig::default_configs(),
        };
        users.insert(user_id, new_user.clone());

        self.save_file(&users).await?;
        Ok(new_user)
    }
}

pub fn hash_username(username: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&username.into_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}

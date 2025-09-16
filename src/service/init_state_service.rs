use crate::{
    managers::{Manager, get_manager},
    models::User,
    state_app::SharedStateApp,
    storage::SharedStorage,
};

use super::Service;

pub struct InitStateService {
    state: SharedStateApp,
    storage: SharedStorage,
}

impl InitStateService {
    pub fn new(state: SharedStateApp, storage: SharedStorage) -> Self {
        Self { state, storage }
    }

    async fn get_user_by_username(&self, username: String) -> Option<User> {
        match self.storage.load(username).await {
            Ok(u) => u,
            Err(_) => None,
        }
    }
}

#[async_trait::async_trait]
impl Service for InitStateService {
    async fn exec(&mut self) {
        let manager = get_manager();
        let current_username = manager.get_username().await;

        let state = self.state.write().await;

        if let Ok(username) = current_username {
            let user = self.get_user_by_username(username.clone()).await;
        }
    }
}

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
        let system_user = manager.get_username().await.ok();
        let mut state = self.state.write().await;

        if system_user.is_none() && state.user.is_some() {
            state.clear_state();
            return;
        }

        if (state.user != system_user) && system_user.is_some() {
            let username = system_user.unwrap();
            match self.get_user_by_username(username.clone()).await {
                Some(user) => {
                    state.user = Some(username);
                    state.config = Some(user.config);
                    state.time_blocks = user.blocks;
                }

                None => {
                    let user = self.storage.create(username.clone()).await.unwrap();
                    state.user = Some(username);
                    state.config = Some(user.config);
                    state.time_blocks = user.blocks;
                }
            }
        }
    }
}

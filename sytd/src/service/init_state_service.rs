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

    async fn create_user_by_no_exist(&self, username: String) -> User {
        if let Some(user) = self.get_user_by_username(username.clone()).await {
            user
        } else {
            let user = self.storage.create(username.clone()).await.unwrap();
            user
        }
    }
}

#[async_trait::async_trait]
impl Service for InitStateService {
    async fn exec(&mut self) {
        let manager = get_manager();
        let system_user = manager.get_username().await.ok();
        let mut state = self.state.write().await;

        match ( state.user.clone(), system_user ) {
            (None, Some(username)) => {
                let new_user = self.create_user_by_no_exist(username.clone()).await;
                state.user = Some(new_user);
                state.active_time_blocks = vec![];
            }

            (Some(user), Some(username)) => {
                if user.username != username {
                    let new_user = self.create_user_by_no_exist(username.clone()).await;
                    state.user = Some(new_user);
                    state.active_time_blocks = vec![];
                }
            }

            (Some(_), None) => {
                state.clear_state();
            }

            _ => {}
        }

    }
}

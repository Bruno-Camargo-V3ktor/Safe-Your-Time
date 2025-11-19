use super::{BuildService, Service, ServicePool};
use crate::{
    managers::SharedManager, state_app::SharedStateApp, storage::SharedStorage,
};
use std::collections::HashMap;
use syt_models::User;

pub struct InitStateService {
    state: SharedStateApp,
    storage: SharedStorage,
    manager: SharedManager,
}

pub struct BuildInitStateService;

#[async_trait::async_trait]
impl BuildService for BuildInitStateService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = InitStateService::new(
            states.get_state::<SharedStateApp>().await.unwrap(),
            states.get_state::<SharedStorage>().await.unwrap(),
            states.get_state::<SharedManager>().await.unwrap(),
        );

        Box::new(service)
    }
}

impl InitStateService {
    pub fn build() -> BuildInitStateService {
        BuildInitStateService {}
    }

    pub fn new(state: SharedStateApp, storage: SharedStorage, manager: SharedManager) -> Self {
        Self {
            state,
            storage,
            manager,
        }
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
        let system_user = self.manager.get_username().await.ok();
        let mut state = self.state.write().await;

        match (state.user.clone(), system_user) {
            (None, Some(username)) => {
                let new_user = self.create_user_by_no_exist(username.clone()).await;

                state.user = Some(new_user);
                state.active_time_blocks = HashMap::new();
            }

            (Some(user), Some(username)) => {
                if user.username != username {
                    let new_user = self.create_user_by_no_exist(username.clone()).await;
                    state.user = Some(new_user);
                    state.active_time_blocks = HashMap::new();
                }
            }

            (Some(_), None) => {
                state.clear_state();
            }

            _ => {}
        }
    }
}

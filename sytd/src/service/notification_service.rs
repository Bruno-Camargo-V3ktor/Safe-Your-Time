use super::{BuildService, Service, ServicePool};
use crate::{managers::SharedManager, state_app::SharedStateApp};
use std::collections::HashMap;
use syt_models::TimeBlock;

pub struct NotificationService {
    state: SharedStateApp,
    manager: SharedManager,
    prev_state: HashMap<String, TimeBlock>,
}

pub struct BuildNotificationService;

impl NotificationService {
    pub fn new(state: SharedStateApp, manager: SharedManager) -> Self {
        Self {
            state,
            manager,
            prev_state: HashMap::new(),
        }
    }

    pub fn build() -> BuildNotificationService {
        BuildNotificationService {}
    }
}

#[async_trait::async_trait]
impl BuildService for BuildNotificationService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = NotificationService::new(
            states.get_state().await.unwrap(),
            states.get_state().await.unwrap(),
        );

        Box::new(service)
    }
}

#[async_trait::async_trait]
impl Service for NotificationService {
    async fn exec(&mut self) {}
}

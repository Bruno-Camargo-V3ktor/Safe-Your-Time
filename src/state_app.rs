use crate::models::{AppConfig, TimeBlock};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type StateAppArc = Arc<RwLock<StateApp>>;

pub struct StateApp {
    user: Option<String>,
    config: Option<AppConfig>,
    active_time_block: Option<TimeBlock>,
}

impl StateApp {
    pub fn new() -> Arc<RwLock<Self>> {
        let state = Self {
            user: None,
            config: None,
            active_time_block: None,
        };

        Arc::new(RwLock::new(state))
    }

    pub fn get_user(&self) -> Option<String> {
        self.user.clone()
    }

    pub fn set_user(&mut self, user: Option<String>) {
        self.user = user;
    }

    pub fn get_config(&self) -> Option<AppConfig> {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: Option<AppConfig>) {
        self.config = config;
    }

    pub fn get_active_time_block(&self) -> Option<TimeBlock> {
        self.active_time_block.clone()
    }

    pub fn set_active_time_block(&mut self, time_block: Option<TimeBlock>) {
        self.active_time_block = time_block;
    }
}

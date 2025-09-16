use crate::models::{AppConfig, TimeBlock};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedStateApp = Arc<RwLock<StateApp>>;

pub struct StateApp {
    pub user: Option<String>,
    pub config: Option<AppConfig>,
    pub active_time_block: Option<TimeBlock>,
    pub time_blocks: Vec<TimeBlock>,
}

impl StateApp {
    pub fn new() -> Arc<RwLock<Self>> {
        let state = Self {
            user: None,
            config: None,
            active_time_block: None,
            time_blocks: vec![],
        };

        Arc::new(RwLock::new(state))
    }
}

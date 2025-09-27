use crate::models::{User, TimeBlock};
use std::{sync::Arc, vec};
use tokio::sync::RwLock;

pub type SharedStateApp = Arc<RwLock<StateApp>>;

pub struct StateApp {
    pub user: Option<User>,
    pub active_time_blocks: Vec<TimeBlock>,
}

impl StateApp {
    pub fn new() -> Arc<RwLock<Self>> {
        let state = Self {
            user: None,
            active_time_blocks: vec![],
        };

        Arc::new(RwLock::new(state))
    }

    pub fn clear_state(&mut self) {
        self.user = None;
        self.active_time_blocks.clear();
    }
}

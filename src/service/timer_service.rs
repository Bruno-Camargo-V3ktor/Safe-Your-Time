use crate::{state_app::SharedStateApp, storage::SharedStorage};

use super::Service;

pub struct TimerService {
    state: SharedStateApp,
    storage: SharedStorage,
}

impl TimerService {
    pub fn new(state: SharedStateApp, storage: SharedStorage) -> Self {
        Self { state, storage }
    }
}

#[async_trait::async_trait]
impl Service for TimerService {
    async fn exec(&mut self) {}
}

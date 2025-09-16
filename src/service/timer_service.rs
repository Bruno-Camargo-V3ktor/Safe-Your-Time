use crate::state_app::SharedStateApp;

use super::Service;

pub struct TimerService {
    state: SharedStateApp,
}

impl TimerService {
    pub fn new(state: SharedStateApp) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for TimerService {
    async fn exec(&mut self) {}
}

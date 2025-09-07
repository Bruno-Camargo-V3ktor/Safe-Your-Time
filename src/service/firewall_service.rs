use crate::state_app::SharedStateApp;

use super::Service;

pub struct FirewallService {
    state: SharedStateApp,
}

impl FirewallService {
    pub fn new(state: SharedStateApp) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for FirewallService {
    async fn exec(&mut self) {}
}

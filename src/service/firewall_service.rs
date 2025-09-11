use super::Service;
use crate::{
    managers::{Manager, get_manager},
    state_app::SharedStateApp,
};

pub struct FirewallService {
    state: SharedStateApp,
    created_rules: Vec<String>,
    clear_rules: bool,
    first_exec: bool,
}

impl FirewallService {
    pub fn new(state: SharedStateApp) -> Self {
        Self {
            state,
            created_rules: vec![],
            clear_rules: false,
            first_exec: true,
        }
    }
}

#[async_trait::async_trait]
impl Service for FirewallService {
    async fn exec(&mut self) {
        let app_state = self.state.read().await;
        let manager = get_manager();

        if self.first_exec {
            let _ = manager.firewall_clean_all().await;
            self.first_exec = false;
        }
    }
}

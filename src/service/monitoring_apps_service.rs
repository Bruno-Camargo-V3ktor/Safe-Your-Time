use super::Service;
use crate::{
    managers::{Manager, get_manager},
    state_app::StateAppArc,
};

pub struct MonitoringAppsService {
    state: StateAppArc,
}

impl MonitoringAppsService {
    pub fn new(state: StateAppArc) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for MonitoringAppsService {
    async fn exec(&mut self) {
        let app_state = self.state.read().await;
        if let Some(config) = app_state.get_config() {
            if let Some(time_block) = app_state.get_active_time_block() {
                let mut apps = time_block.denied_apps.clone();
                apps.append(&mut config.default_denied_apps.clone());
                get_manager().monitoring_apps(apps).await;
            }
        }
    }
}

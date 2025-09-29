use std::collections::HashSet;

use super::Service;
use crate::{ managers::{ Manager, get_manager }, models::StateBlock, state_app::SharedStateApp };

pub struct MonitoringAppsService {
    state: SharedStateApp,
}

impl MonitoringAppsService {
    pub fn new(state: SharedStateApp) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for MonitoringAppsService {
    async fn exec(&mut self) {
        let app_state = self.state.read().await;

        if let Some(user) = &app_state.user {
            for (_, time_block) in &app_state.active_time_blocks {
                if time_block.state != StateBlock::InProgress {
                    continue;
                }

                let mut apps = time_block.denied_apps.clone();
                apps.append(&mut user.config.default_denied_apps.clone());

                let remove_set: HashSet<_> = time_block.allow_apps.clone().into_iter().collect();
                apps.retain(|x| !remove_set.contains(x));

                get_manager().monitoring_apps(apps).await;
            }
        }
    }
}

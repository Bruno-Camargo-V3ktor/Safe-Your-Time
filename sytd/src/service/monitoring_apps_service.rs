use super::{BuildService, Service, ServicePool};
use crate::{managers::SharedManager, models::StateBlock, state_app::SharedStateApp};
use std::collections::HashSet;

pub struct MonitoringAppsService {
    state: SharedStateApp,
    manager: SharedManager,
}

pub struct BuildMonitoringAppsService;

#[async_trait::async_trait]
impl BuildService for BuildMonitoringAppsService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = MonitoringAppsService::new(
            states.get_state::<SharedStateApp>().await.unwrap(),
            states.get_state::<SharedManager>().await.unwrap(),
        );

        Box::new(service)
    }
}

impl MonitoringAppsService {
    pub fn build() -> BuildMonitoringAppsService {
        BuildMonitoringAppsService {}
    }

    pub fn new(state: SharedStateApp, manager: SharedManager) -> Self {
        Self { state, manager }
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

                self.manager.monitoring_apps(apps).await;
            }
        }
    }
}

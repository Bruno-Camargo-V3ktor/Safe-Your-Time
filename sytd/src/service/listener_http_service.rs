use super::{BuildService, Service, ServicePool};
use crate::communication::{Listener, ListenerHttp, SharedController};

pub struct ListenerHttpService {
    controller: SharedController,
}

pub struct BuildListenerHttpService;

#[async_trait::async_trait]
impl BuildService for BuildListenerHttpService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service =
            ListenerHttpService::new(states.get_state::<SharedController>().await.unwrap());

        Box::new(service)
    }
}

impl ListenerHttpService {
    pub fn build() -> BuildListenerHttpService {
        BuildListenerHttpService {}
    }

    pub fn new(controller: SharedController) -> Self {
        Self { controller }
    }
}

#[async_trait::async_trait]
impl Service for ListenerHttpService {
    async fn exec(&mut self) {
        let controller = self.controller.clone();
        let listener_http = ListenerHttp::new(controller);
        let _ = listener_http.server("127.0.0.1").await;
    }
}

use super::{BuildService, Service, ServicePool};
use crate::communication::{Listener, ListenerSockter, SharedController};

pub struct ListenerSocketService {
    controller: SharedController,
}

pub struct BuildListenerSocketService;

#[async_trait::async_trait]
impl BuildService for BuildListenerSocketService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service =
            ListenerSocketService::new(states.get_state::<SharedController>().await.unwrap());

        Box::new(service)
    }
}

impl ListenerSocketService {
    pub fn build() -> BuildListenerSocketService {
        BuildListenerSocketService {}
    }

    pub fn new(controller: SharedController) -> Self {
        Self { controller }
    }
}

#[async_trait::async_trait]
impl Service for ListenerSocketService {
    async fn exec(&mut self) {
        let controller = self.controller.clone();
        let listener_serve = ListenerSockter::new(controller);
        let _ = listener_serve.server("/tmp/sytd.sock").await;
    }
}

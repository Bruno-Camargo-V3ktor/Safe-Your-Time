use super::{ Service, BuildService, ServicePool };
use crate::communication::{ Listener, ListenerHttp, SharedController };
use tokio::task::JoinHandle;

pub struct ListenerHttpService {
    controller: SharedController,
    server_handle: Option<JoinHandle<()>>,
}

pub struct BuildListenerHttpService;

#[async_trait::async_trait]
impl BuildService for BuildListenerHttpService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = ListenerHttpService::new(
            states.get_state::<SharedController>().await.unwrap()
        );

        Box::new(service)
    }
}

impl ListenerHttpService {
    pub fn build() -> BuildListenerHttpService {
        BuildListenerHttpService {}
    }

    pub fn new(controller: SharedController) -> Self {
        Self {
            controller,
            server_handle: None,
        }
    }

    fn start_server(&mut self) {
        let controller = self.controller.clone();
        self.server_handle = Some(
            tokio::spawn(async move {
                let listener_http = ListenerHttp::new(controller);
                let _ = listener_http.server("127.0.0.1").await;
            })
        );
    }
}

#[async_trait::async_trait]
impl Service for ListenerHttpService {
    async fn exec(&mut self) {
        match &self.server_handle {
            Some(handle) => {
                if handle.is_finished() {
                    self.start_server();
                }
            }

            None => {
                self.start_server();
            }
        };
    }
}

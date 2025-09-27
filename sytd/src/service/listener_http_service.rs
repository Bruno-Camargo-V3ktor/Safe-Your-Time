use super::Service;
use crate::{
    communication::{Listener, ListenerHttp, SharedController},
    state_app::SharedStateApp,
};
use tokio::task::JoinHandle;

pub struct ListenerHttpService {
    state: SharedStateApp,
    controller: SharedController,
    server_handle: Option<JoinHandle<()>>,
}

impl ListenerHttpService {
    pub fn new(state: SharedStateApp, controller: SharedController) -> Self {
        Self {
            state,
            controller,
            server_handle: None,
        }
    }

    fn start_server(&mut self) {
        let controller = self.controller.clone();
        self.server_handle = Some(tokio::spawn(async move {
            let listener_http = ListenerHttp::new(controller);
            let _ = listener_http.server("127.0.0.1").await;
        }));
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

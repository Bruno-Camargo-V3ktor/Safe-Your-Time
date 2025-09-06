use super::Service;
use crate::{
    communication::{Listener, ListenerSockter, SharedController},
    state_app::SharedStateApp,
};
use tokio::task::JoinHandle;

pub struct ListenerSocketService {
    state: SharedStateApp,
    controller: SharedController,
    handle_server: Option<JoinHandle<()>>,
}

impl ListenerSocketService {
    pub fn new(state_app: SharedStateApp, controller: SharedController) -> Self {
        Self {
            state: state_app,
            controller,
            handle_server: None,
        }
    }

    pub fn init_server(&mut self) {
        let controller = self.controller.clone();
        self.handle_server = Some(tokio::spawn(async move {
            let listener_serve = ListenerSockter::new(controller);
            let _ = listener_serve.server("/tmp/sytd.sock").await;
        }));
    }
}

#[async_trait::async_trait]
impl Service for ListenerSocketService {
    async fn exec(&mut self) {
        match &self.handle_server {
            Some(handle) => {
                if handle.is_finished() {
                    self.init_server();
                }
            }

            None => self.init_server(),
        };
    }
}

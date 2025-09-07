use actix_web::{App, HttpServer};
use tokio::task::JoinHandle;

use crate::{communication::SharedController, state_app::SharedStateApp};

use super::Service;

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
        self.server_handle = Some(tokio::spawn(async move {
            let _ = HttpServer::new(|| App::new().route("/hey", actix_web::web::get().to(hey)))
                .bind(("127.0.0.1", 4321))
                .unwrap()
                .run()
                .await;
        }));
    }
}

#[async_trait::async_trait]
impl Service for ListenerHttpService {
    async fn exec(&mut self) {
        let is_enable_http = if let Some(config) = &self.state.read().await.get_config() {
            config.http_listening
        } else {
            true
        };

        match &self.server_handle {
            Some(handle) => {
                if is_enable_http && handle.is_finished() {
                    self.start_server();
                } else if !is_enable_http {
                    handle.abort();
                }
            }

            None => {
                if is_enable_http {
                    self.start_server();
                }
            }
        };
    }
}

async fn hey() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("hey")
}

use super::{Listener, controller::SharedController};
use actix_web::{App, HttpResponse, HttpServer, Responder};

pub struct ListenerHttp {
    controller: SharedController,
}

impl ListenerHttp {
    pub fn new(controller: SharedController) -> Self {
        Self { controller }
    }
}

#[async_trait::async_trait]
impl Listener for ListenerHttp {
    fn get_controller(&self) -> SharedController {
        self.controller.clone()
    }

    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()> {
        let _ = HttpServer::new(|| App::new().route("/hey", actix_web::web::get().to(hey)))
            .bind((addr.into(), 4321))
            .unwrap()
            .run()
            .await;

        Ok(())
    }
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey")
}

use super::super::{Listener, controller::SharedController};
use actix_web::{App, HttpServer, web};

use super::routers::delete_time_block;

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
        let controller = self.controller.clone();
        let _ = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(controller.clone()))
                .service(web::scope("/api").service(delete_time_block))
        })
        .bind((addr.into(), 4321))
        .unwrap()
        .run()
        .await;

        Ok(())
    }
}

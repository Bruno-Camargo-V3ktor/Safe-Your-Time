use crate::utils::get_dir;
use actix_files as fs;
use super::{SharedController, Listener};
use actix_web::{ App, HttpServer, Responder, get, web };
use super::routers::{
    create_time_block,
    update_time_block,
    delete_time_block,
    get_time_bock,
    list_time_bock,
    command_for_time_block,
    get_config,
    update_config,
};
use std::path::PathBuf;

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
        let current_dir = get_dir();
        let mut directory = PathBuf::from(&current_dir);
        directory.push("pages");

        let controller = self.controller.clone();
        let _ = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(controller.clone()))
                .service(
                    web
                        ::scope("/api")
                        .service(create_time_block)
                        .service(update_time_block)
                        .service(delete_time_block)
                        .service(get_time_bock)
                        .service(list_time_bock)
                        .service(command_for_time_block)
                        .service(get_config)
                        .service(update_config)
                )
                .service(index)
                .service(fs::Files::new("/", directory.clone()))
        })
            .bind((addr.into(), 4321))
            .unwrap()
            .run().await;

        Ok(())
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let current_dir = get_dir();
    let mut directory = PathBuf::from(&current_dir);
    directory.push("pages");
    directory.push("index.html");

    fs::NamedFile::open(directory).unwrap()
}

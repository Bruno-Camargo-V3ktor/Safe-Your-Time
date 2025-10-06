use actix_web::{ Responder, delete, web, get };
use serde::Deserialize;
use crate::communication::{
    Commands,
    DeleteTimeBlockArgs,
    SharedController,
    ShowTimeBlockArgs,
    http::util::converte_response_in_http,
};

#[derive(Deserialize)]
struct FilterActives {
    actives: Option<bool>,
}

#[delete("/timeblock/{name}")]
pub async fn delete_time_block(
    controller: web::Data<SharedController>,
    name: web::Path<String>
) -> impl Responder {
    let name = name.into_inner();

    let command = Commands::DeleteTimeBlock(DeleteTimeBlockArgs { name });
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 404, 500)
}

#[get("/timeblock/{name}")]
pub async fn get_time_bock(
    controller: web::Data<SharedController>,
    name: web::Path<String>
) -> impl Responder {
    let name = name.into_inner();

    let command = Commands::ShowTimeBlock(ShowTimeBlockArgs { name });
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 404, 500)
}

#[get("/timeblock")]
pub async fn list_time_bock(
    controller: web::Data<SharedController>,
    show_actives: web::Query<FilterActives>
) -> impl Responder {
    let show_actives = show_actives.into_inner().actives.unwrap_or_default();

    let command = if show_actives {
        Commands::ShowActiveTimeBlocks
    } else {
        Commands::ListTimeBlocks
    };

    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 404, 500)
}

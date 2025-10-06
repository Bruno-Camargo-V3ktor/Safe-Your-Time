use actix_web::{ HttpResponse, Responder, delete, get, patch, post, web };
use serde::Deserialize;
use serde_json::json;
use crate::communication::{
    Commands,
    CreateTimeBlockArgs,
    DeleteTimeBlockArgs,
    PauseTimeBlockArgs,
    ResponseContent,
    SharedController,
    ShowTimeBlockArgs,
    StartTimeBlockArgs,
    StopTimeBlockArgs,
    UpdateConfigArgs,
    UpdateTimeBlockArgs,
    http::util::converte_response_in_http,
};

#[derive(Deserialize)]
struct FilterActives {
    actives: Option<bool>,
}

#[derive(Deserialize)]
struct CommandForTimeBlock {
    command: String,
}

#[post("/timeblock")]
pub async fn create_time_block(
    controller: web::Data<SharedController>,
    content: web::Json<CreateTimeBlockArgs>
) -> impl Responder {
    let command = Commands::CreateTimeBlock(content.into_inner());
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 400, 500)
}

#[patch("/timeblock")]
pub async fn update_time_block(
    controller: web::Data<SharedController>,
    content: web::Json<UpdateTimeBlockArgs>
) -> impl Responder {
    let command = Commands::UpdateTimeBlock(content.into_inner());
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 400, 500)
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

#[post("/timeblock/{name}")]
pub async fn command_for_time_block(
    controller: web::Data<SharedController>,
    name: web::Path<String>,
    command: web::Query<CommandForTimeBlock>
) -> impl Responder {
    let name = name.into_inner();

    let command = match &command.into_inner().command[..] {
        "start" => { Commands::StartTimeBlock(StartTimeBlockArgs { name }) }
        "stop" => { Commands::StopTimeBlock(StopTimeBlockArgs { name }) }
        "pause" => { Commands::PauseTimeBlock(PauseTimeBlockArgs { name }) }

        _ => {
            return HttpResponse::BadRequest().json(ResponseContent {
                message: String::from("Command not exist"),
                payload: json!({}),
            });
        }
    };
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 404, 500)
}

#[get("/config")]
pub async fn get_config(controller: web::Data<SharedController>) -> impl Responder {
    let command = Commands::ShowConfig;
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 400, 500)
}

#[patch("/config")]
pub async fn update_config(
    controller: web::Data<SharedController>,
    content: web::Json<UpdateConfigArgs>
) -> impl Responder {
    let command = Commands::UpdateConfig(content.into_inner());
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 400, 500)
}

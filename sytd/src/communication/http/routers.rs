use actix_web::{Responder, delete, web};

use crate::communication::{
    Commands, DeleteTimeBlockArgs, SharedController, http::util::converte_response_in_http,
};

#[delete("/timeblock/{name}")]
pub async fn delete_time_block(
    controller: web::Data<SharedController>,
    name: web::Path<String>,
) -> impl Responder {
    let name = name.into_inner();

    let command = Commands::DeleteTimeBlock(DeleteTimeBlockArgs { name });
    let response = controller.process(command).await;

    converte_response_in_http(response, 200, 404, 500)
}

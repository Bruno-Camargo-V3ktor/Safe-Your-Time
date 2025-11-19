use crate::managers::SharedManager;
use actix_web::{ HttpResponse, HttpResponseBuilder, http::StatusCode };
use syt_communication::Responses;

pub fn get_dir() -> String {
    std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string()
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix
            ::signal(tokio::signal::unix::SignalKind::terminate())
            .unwrap()
            .recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}

pub fn converte_response_in_http(
    response: Responses,
    success_code: u16,
    error_code: u16,
    panic_code: u16
) -> HttpResponse {
    let body;
    let status_code: u16;

    match response {
        Responses::Success(content) => {
            body = content;
            status_code = success_code;
        }

        Responses::Error(content) => {
            body = content;
            status_code = error_code;
        }

        Responses::Panic(content) => {
            body = content;
            status_code = panic_code;
        }
    }

    HttpResponseBuilder::new(StatusCode::from_u16(status_code).unwrap()).json(body)
}

pub fn start_timeblock_notification(manager: &SharedManager, name: String, mensagem: String) {
    let _ = manager.notification(
        String::from("Start"),
        name,
        mensagem,
        String::from("warning.png")
    );
}

pub fn stop_timeblock_notification(manager: &SharedManager, name: String) {
    let _ = manager.notification(
        String::from("Completed"),
        name,
        String::from(""),
        String::from("warning.png")
    );
}

pub fn pause_timeblock_notification(manager: &SharedManager, name: String) {
    let _ = manager.notification(
        String::from("Paused"),
        name,
        String::from(""),
        String::from("warning.png")
    );
}

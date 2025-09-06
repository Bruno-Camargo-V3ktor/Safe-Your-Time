use crate::{
    communication::{Listener, ListenerSockter},
    service::{MonitoringAppsService, ServiceController},
    state_app::StateApp,
    storage::{Storage, SurrealDbStorage},
    utils::{get_dir, shutdown_signal},
};
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};

mod communication;
mod managers;
mod models;
mod service;
mod state_app;
mod storage;
mod utils;

#[tokio::main]
async fn main() {
    let state_app = StateApp::new();
    let mut services = ServiceController::new();
    let storage = Box::new(SurrealDbStorage::new(&get_dir(), "sytd-ns", "sytd-db").await);
    let controller = communication::Controller::new(storage.clone(), state_app.clone());

    services.add_service(MonitoringAppsService::new(state_app.clone()), 5000);
    services.init().await;

    let mut socket_listener_handle = spawn_socket_listener(storage.clone(), state_app.clone());

    shutdown_signal().await
}

fn spawn_socket_listener(
    storage: Box<dyn Storage + Send + Sync>,
    state: Arc<RwLock<StateApp>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let controller = communication::Controller::new(storage, state);
        let listener_serve = ListenerSockter::new(controller);
        let _ = listener_serve.server("/tmp/sytd.sock").await;
    })
}

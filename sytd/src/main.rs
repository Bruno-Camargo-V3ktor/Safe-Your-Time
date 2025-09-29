use crate::{
    communication::Controller,
    service::{
        InitStateService,
        ListenerHttpService,
        ListenerSocketService,
        MonitoringAppsService,
        ServicePool,
        TimerService,
    },
    state_app::StateApp,
    storage::JsonStorage,
    utils::{ get_dir, shutdown_signal },
};

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
    let storage = JsonStorage::new(&get_dir()).await;
    let controller = Controller::new(storage.clone(), state_app.clone());

    let mut services = ServicePool::new();
    services.add_service(InitStateService::new(state_app.clone(), storage.clone()), 2500);

    services.add_service(TimerService::new(state_app.clone()), 10000);

    services.add_service(MonitoringAppsService::new(state_app.clone()), 5000);

    services.add_service(ListenerSocketService::new(state_app.clone(), controller.clone()), 5000);
    services.add_service(ListenerHttpService::new(state_app.clone(), controller.clone()), 10000);
    services.init().await;

    shutdown_signal().await
}

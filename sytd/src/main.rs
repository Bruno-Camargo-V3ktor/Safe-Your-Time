use crate::{
    communication::Controller,
    service::{
        BuildService,
        InitStateService,
        ListenerHttpService,
        ListenerSocketService,
        MonitoringAppsService,
        ServicePool,
        TimerService,
    },
    state_app::StateApp,
    storage::{ JsonStorage, SharedStorage, Storage },
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
    services.add_state(state_app).await;
    services.add_state(storage).await;
    services.add_state(controller).await;

    services.add_service(InitStateService::build(), 5000);
    services.add_service(TimerService::build(), 2500);
    services.add_service(MonitoringAppsService::new(state_app.clone()), 5000);
    services.add_service(ListenerSocketService::new(controller.clone()), 10000);
    services.add_service(ListenerHttpService::new(controller.clone()), 10000);

    services.init().await;

    shutdown_signal().await
}

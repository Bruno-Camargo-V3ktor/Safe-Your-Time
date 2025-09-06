use crate::{
    communication::{Listener, ListenerSockter},
    models::AppConfig,
    storage::{Storage, SurrealDbStorage},
    utils::{get_dir, shutdown_signal},
};
use managers::{Manager, get_manager};
use models::TimeBlock;
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, task::JoinHandle, time::sleep};

mod communication;
mod managers;
mod models;
mod service;
mod storage;
mod utils;

pub struct StateApp {
    user: Option<String>,
    config: Option<AppConfig>,
    active_time_block: Option<TimeBlock>,
}

#[tokio::main]
async fn main() {
    let state_app = Arc::new(RwLock::new(StateApp {
        user: get_manager().get_username().await.ok(),
        config: None,
        active_time_block: None,
    }));
    let storage = Box::new(SurrealDbStorage::new(&get_dir(), "sytd-ns", "sytd-db").await);
    //let controller = communication::Controller::new(storage, state);

    let mut socket_listener_handle = spawn_socket_listener(storage.clone(), state_app.clone());
    let mut monitoring_apps_handle = spawn_monitoting_apps(state_app.clone());

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

fn spawn_monitoting_apps(state: Arc<RwLock<StateApp>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut time = 5000;
        loop {
            let app_state = state.read().await;
            if let Some(config) = &app_state.config {
                time = config.monitoring_time;

                if let Some(time_block) = &app_state.active_time_block {
                    let mut apps = time_block.denied_apps.clone();
                    apps.append(&mut config.default_denied_apps.clone());
                    get_manager().monitoring_apps(apps).await;
                }
            }
            sleep(Duration::from_millis(time as u64)).await;
        }
    })
}

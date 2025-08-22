use std::time::Duration;

use managers::{Manager, get_manager};
use tokio::time::sleep;

use crate::communication::{Listener, ListenerSockter};

mod communication;
mod managers;
mod models;

#[tokio::main]
async fn main() {
    let server_handle = tokio::spawn(async move {
        let controller = communication::Controller {};
        let listener_serve = ListenerSockter::new(controller);
        let _ = listener_serve.server("/tmp/sytd.sock").await;
    });

    let monitoring_apps_handle = tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(5000)).await;
            get_manager()
                .monitoring_apps(vec!["librum".to_string()])
                .await;
        }
    });

    let _ = tokio::join!(server_handle, monitoring_apps_handle);
}

use managers::{Manager, get_manager};

mod managers;
mod models;

#[tokio::main]
async fn main() {
    let manager = get_manager();
    manager.monitoring_apps(vec!["librum".to_string()]).await;
}

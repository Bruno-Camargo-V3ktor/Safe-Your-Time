use std::time::Duration;

mod listener_socket_service;
mod monitoring_apps_service;
pub use listener_socket_service::*;
pub use monitoring_apps_service::*;

#[async_trait::async_trait]
pub trait Service {
    async fn exec(&mut self);
}

pub struct ServiceController {
    services: Vec<(Box<dyn Service + Send + Sync>, Duration)>,
}

impl ServiceController {
    pub fn new() -> Self {
        Self { services: vec![] }
    }

    pub fn add_service<S: Service + Send + Sync + 'static>(&mut self, service: S, time: u64) {
        self.services
            .push((Box::new(service), Duration::from_millis(time)));
    }

    pub async fn init(self) {
        for (mut service, time) in self.services.into_iter() {
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(time);

                loop {
                    interval.tick().await;
                    service.exec().await;
                }
            });
        }
    }
}

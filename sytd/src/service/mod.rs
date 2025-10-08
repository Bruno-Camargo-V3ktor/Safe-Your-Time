use std::{ any::{ Any, TypeId }, collections::HashMap, sync::Arc, time::Duration };
use tokio::sync::RwLock;

//mod firewall_service;
mod init_state_service;
mod listener_http_service;
mod listener_socket_service;
mod monitoring_apps_service;
mod timer_service;

//pub use firewall_service::*;
pub use init_state_service::*;
pub use listener_http_service::*;
pub use listener_socket_service::*;
pub use monitoring_apps_service::*;
pub use timer_service::*;

#[async_trait::async_trait]
pub trait Service {
    async fn exec(&mut self);
}

pub struct ServicePool {
    map_state: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Sync + Send>>>>,
    services: Vec<(Box<dyn Service + Send + Sync>, Duration)>,
}

impl ServicePool {
    pub fn new() -> Self {
        Self { services: vec![], map_state: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn add_service<S: Service + Send + Sync + 'static>(&mut self, service: S, time: u64) {
        self.services.push((Box::new(service), Duration::from_millis(time)));
    }

    pub async fn add_state<T>(&self, value: T) where T: 'static + Send + Sync + Clone {
        let mut write = self.map_state.write().await;
        write.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub async fn get_state<T>(&self) -> Option<T> where T: 'static + Send + Sync + Clone {
        let read = self.map_state.read().await;
        read.get(&TypeId::of::<T>()).and_then(|boxed_any| boxed_any.downcast_ref::<T>().cloned())
    }

    pub async fn init(self) {
        for (service, time) in self.services.into_iter() {
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(time.clone());
                let mut service = service;

                loop {
                    interval.tick().await;
                    service.exec().await;
                }
            });
        }
    }
}

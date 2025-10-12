use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::{sync::RwLock, task::JoinHandle};

//mod firewall_service;
mod init_state_service;
mod listener_http_service;
mod listener_socket_service;
mod monitoring_apps_service;
mod notification_service;
mod timer_service;

//pub use firewall_service::*;
pub use init_state_service::*;
pub use listener_http_service::*;
pub use listener_socket_service::*;
pub use monitoring_apps_service::*;
pub use notification_service::*;
pub use timer_service::*;

#[async_trait::async_trait]
pub trait Service {
    async fn exec(&mut self);
}

#[async_trait::async_trait]
pub trait BuildService {
    async fn build(&self, service: &ServicePool) -> Box<dyn Service + Send + Sync>;
}

pub struct ServicePool {
    map_state: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Sync + Send>>>>,
    services: Vec<(Box<dyn BuildService + Send + Sync>, Duration)>,
}

impl ServicePool {
    pub fn new() -> Self {
        Self {
            services: vec![],
            map_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_service<S: BuildService + Send + Sync + 'static>(
        &mut self,
        build_service: S,
        time: u64,
    ) {
        self.services
            .push((Box::new(build_service), Duration::from_millis(time)));
    }

    pub async fn add_state<T>(&self, value: T)
    where
        T: 'static + Send + Sync + Clone,
    {
        let mut write = self.map_state.write().await;
        write.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub async fn get_state<T>(&self) -> Option<T>
    where
        T: 'static + Send + Sync + Clone,
    {
        let read = self.map_state.read().await;
        read.get(&TypeId::of::<T>())
            .and_then(|boxed_any| boxed_any.downcast_ref::<T>().cloned())
    }

    pub async fn run(self) {
        let _ = tokio::spawn(async move {
            let pool = self;
            let mut handlers: Vec<Option<JoinHandle<()>>> = vec![];
            pool.services.iter().for_each(|_| handlers.push(None));

            loop {
                let services = &pool.services;
                for i in 0..services.len() {
                    if let Some(handle) = handlers.get(i) {
                        if handle.is_some() && !handle.as_ref().unwrap().is_finished() {
                            continue;
                        }
                    }

                    let (build_service, duration) = services.get(i).unwrap();

                    let mut service = build_service.build(&pool).await;
                    let sleep = tokio::time::sleep(duration.clone());

                    let h = tokio::spawn(async move {
                        service.exec().await;
                        sleep.await;
                    });

                    handlers[i] = Some(h);
                }
            }
        });
    }
}

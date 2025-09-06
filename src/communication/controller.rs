use super::{Responses, commands::Commands};
use crate::{state_app::SharedStateApp, storage::Storage};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedController = Arc<RwLock<Controller>>;

pub struct Controller {
    storage: Box<dyn Storage + Send + Sync>,
    state: SharedStateApp,
}

impl Controller {
    pub fn new(storage: Box<dyn Storage + Send + Sync>, state: SharedStateApp) -> SharedController {
        let controller = Self { storage, state };
        Arc::new(RwLock::new(controller))
    }

    pub async fn process(&self, command: Commands) -> Responses {
        Responses::Success(String::from("Hi"))
    }
}

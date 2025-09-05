use super::{Responses, commands::Commands};
use crate::{StateApp, models::TimeBlock, storage::Storage};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Controller {
    storage: Box<dyn Storage + Send + Sync>,
    state: Arc<RwLock<StateApp>>,
}

impl Controller {
    pub fn new(storage: Box<dyn Storage + Send + Sync>, state: Arc<RwLock<StateApp>>) -> Self {
        Self { storage, state }
    }

    pub async fn process(&self, command: Commands) -> Responses {
        Responses::Success(String::from("Hi"))
    }
}

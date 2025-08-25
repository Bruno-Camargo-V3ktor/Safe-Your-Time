use super::{Responses, commands::Commands};
use crate::{models::TimeBlock, storage::Storage};
use std::sync::Arc;
use tokio::sync::RwLock;

/// username, id, timeblock
type AppState = Arc<RwLock<(String, (String, Option<TimeBlock>))>>;

pub struct Controller {
    storage: Box<dyn Storage + Send + Sync>,
    state: AppState,
}

impl Controller {
    pub fn new(storage: Box<dyn Storage + Send + Sync>, state: AppState) -> Self {
        Self { storage, state }
    }

    pub async fn process(&self, command: Commands) -> Responses {
        Responses::Success(String::from("Hi"))
    }
}

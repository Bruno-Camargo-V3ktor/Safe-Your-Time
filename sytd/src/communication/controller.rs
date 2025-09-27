use super::{ Responses, commands::Commands };
use crate::{ models::TimeBlock, state_app::SharedStateApp, storage::SharedStorage };
use serde_json::json;
use std::sync::Arc;

pub type SharedController = Arc<Controller>;

pub struct Controller {
    storage: SharedStorage,
    state: SharedStateApp,
}

impl Controller {
    pub fn new(storage: SharedStorage, state: SharedStateApp) -> SharedController {
        let controller = Self { storage, state };
        Arc::new(controller)
    }

    pub async fn process(&self, command: Commands) -> Responses {
        match command {
            Commands::CreateTimeBlock(args) => {
                let mut state = self.state.write().await;
                let storage = self.storage.clone();

                let new_time_block = TimeBlock {
                    ..args
                };

                 Responses::error("commando not implemation".to_string(), json!({}))
            },

            _ => Responses::error("commando not implemation".to_string(), json!({})),
        }
    }
}

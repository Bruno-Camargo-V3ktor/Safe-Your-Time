use super::{Responses, TypeReponses, commands::Commands};
use crate::{state_app::SharedStateApp, storage::SharedStorage};
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
            _ => Responses::new(
                TypeReponses::Error,
                "commando not implemation".to_string(),
                json!({}),
            ),
        }
    }
}

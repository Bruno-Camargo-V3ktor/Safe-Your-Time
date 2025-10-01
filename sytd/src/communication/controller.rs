use super::{Responses, commands::Commands};
use crate::{
    communication::CreateTimeBlockArgs, models::TimeBlock, state_app::SharedStateApp,
    storage::SharedStorage,
};
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
            Commands::CreateTimeBlock(args) => self.create_time_block(args).await,

            _ => Responses::error("commando not implemation".to_string(), json!({})),
        }
    }

    async fn create_time_block(&self, args: CreateTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;
        let storage = self.storage.clone();

        if let Some(user) = state.user.as_mut() {
            if user.blocks.contains_key(&args.name) {
                return Responses::error(
                    "There is already a time block with that name".to_string(),
                    json!({}),
                );
            }

            let mut tb_builder = TimeBlock::new();
            tb_builder.name(args.name);
            tb_builder.message(args.message);
            tb_builder.duration(args.duration);
            tb_builder.time(args.start_time, args.end_time);
            tb_builder.allow(args.allow_web, args.allow_apps);
            tb_builder.denied(args.denied_web, args.denied_apps);
            tb_builder.days(args.days);

            return match tb_builder.build() {
                Ok(tb) => {
                    user.blocks.insert(tb.name.clone(), tb.clone());
                    let _ = storage.save(user).await;
                    Responses::success("TimeBlock created successfully".to_string(), tb)
                }
                Err(msg) => Responses::error(msg, json!({})),
            };
        }

        Responses::error("No user logged in".to_string(), json!({}))
    }
}

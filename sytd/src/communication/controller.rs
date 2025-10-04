use super::{Responses, commands::Commands};
use crate::{
    communication::{
        CreateTimeBlockArgs, DeleteTimeBlockArgs, ShowTimeBlockArgs, UpdateTimeBlockArgs,
    },
    models::{StateBlock, TimeBlock},
    state_app::SharedStateApp,
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
            Commands::UpdateTimeBlock(args) => self.update_time_block(args).await,
            Commands::DeleteTimeBlock(args) => self.delete_time_block(args).await,
            Commands::ShowTimeBlock(args) => self.get_time_bock(args).await,

            Commands::ListTimeBlocks => self.list_all_time_blocks().await,
            Commands::ShowActiveTimeBlocks => self.list_active_time_blocks().await,

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

    async fn update_time_block(&self, args: UpdateTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;
        let storage = self.storage.clone();

        if let Some(user) = state.user.as_mut() {
            if !user.blocks.contains_key(&args.name) {
                return Responses::error("Time block not found".to_string(), json!({}));
            }

            if args.new_name.is_some() && user.blocks.contains_key(&args.new_name.unwrap()) {
                return Responses::error(
                    "Name already used by another TimeBlock".to_string(),
                    json!({}),
                );
            }

            let _ = user.blocks.remove(&args.name);

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
                    Responses::success("TimeBlock updated successfully".to_string(), tb)
                }
                Err(msg) => Responses::error(msg, json!({})),
            };
        }

        Responses::error("No user logged in".to_string(), json!({}))
    }

    async fn delete_time_block(&self, args: DeleteTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;
        let storage = self.storage.clone();
        let mut success_deleted = false;

        if let Some(user) = state.user.as_mut() {
            if !user.blocks.contains_key(&args.name) {
                return Responses::error("Time block not found".to_string(), json!({}));
            }

            let _ = user.blocks.remove(&args.name);
            let _ = storage.save(user).await;
            success_deleted = true;
        }

        if success_deleted {
            let _ = state.active_time_blocks.remove(&args.name);
            Responses::success("TimeBlock deleted successfully".to_string(), json!({}));
        }

        Responses::error("No user logged in".to_string(), json!({}))
    }

    async fn get_time_bock(&self, args: ShowTimeBlockArgs) -> Responses {
        let state = self.state.read().await;

        if let Some(user) = state.user.as_ref() {
            return match user.blocks.get(&args.name) {
                Some(tb) => Responses::success("Success".to_string(), tb),
                None => Responses::error("Time block not found".to_string(), json!({})),
            };
        }

        Responses::error("No user logged in".to_string(), json!({}))
    }

    async fn list_all_time_blocks(&self) -> Responses {
        let state = self.state.read().await;

        if let Some(user) = state.user.as_ref() {
            let list = user.blocks.iter().map(|(_, tb)| tb).collect::<Vec<_>>();
            Responses::success("Success".to_string(), list);
        }

        Responses::error("No user logged in".to_string(), json!({}))
    }

    async fn list_active_time_blocks(&self) -> Responses {
        let state = self.state.read().await;
        if state.user.is_none() {
            return Responses::error("No user logged in".to_string(), json!({}));
        }

        let list = state
            .active_time_blocks
            .iter()
            .map(|(_, tb)| tb)
            .filter(|tb| tb.state == StateBlock::InProgress)
            .collect::<Vec<_>>();
        Responses::success("Success".to_string(), list)
    }
}

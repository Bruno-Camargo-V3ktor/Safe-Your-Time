use super::{Responses, commands::Commands};
use crate::{
    communication::{
        CreateTimeBlockArgs, DeleteTimeBlockArgs, PauseTimeBlockArgs, ShowTimeBlockArgs,
        StartTimeBlockArgs, StopTimeBlockArgs, UpdateConfigArgs, UpdateTimeBlockArgs,
    },
    
    state_app::SharedStateApp,
    storage::SharedStorage,
};
use syt_models::{StateBlock, TimeBlock, TimeRegister};
use chrono::{Duration, Local, Timelike};
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

            Commands::StartTimeBlock(args) => self.start_time_block(args).await,
            Commands::PauseTimeBlock(args) => self.toggle_pause_time_block(args).await,
            Commands::StopTimeBlock(args) => self.stop_time_block(args).await,

            Commands::ShowConfig => self.get_cofig().await,
            Commands::UpdateConfig(args) => self.update_cofig(args).await,
            //_ => Responses::error("commando not implemation".to_string(), json!({})),
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

        Responses::panic("No user logged in".to_string(), json!({}))
    }

    async fn update_time_block(&self, mut args: UpdateTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;
        let storage = self.storage.clone();

        let mut success_update = false;
        let mut updated_tb = None;

        if let Some(user) = state.user.as_mut() {
            if !user.blocks.contains_key(&args.name) {
                return Responses::error("Time block not found".to_string(), json!({}));
            }

            if args.new_name.is_some() && user.blocks.contains_key(&args.new_name.clone().unwrap())
            {
                return Responses::error(
                    "Name already used by another TimeBlock".to_string(),
                    json!({}),
                );
            }

            let _ = user.blocks.remove(&args.name);

            args.new_name = Some(args.new_name.clone().unwrap_or(args.name.clone()));

            let mut tb_builder = TimeBlock::new();
            tb_builder.name(args.new_name.unwrap());
            tb_builder.message(args.message);
            tb_builder.duration(args.duration);
            tb_builder.time(args.start_time, args.end_time);
            tb_builder.allow(args.allow_web, args.allow_apps);
            tb_builder.denied(args.denied_web, args.denied_apps);
            tb_builder.days(args.days);

            match tb_builder.build() {
                Ok(tb) => {
                    user.blocks.insert(tb.name.clone(), tb.clone());
                    let _ = storage.save(user).await;
                    updated_tb = Some(tb);
                    success_update = true;
                }
                Err(msg) => {
                    return Responses::error(msg, json!({}));
                }
            };
        }

        if success_update {
            let _ = state.active_time_blocks.remove(&args.name);
            return Responses::success(
                "TimeBlock updated successfully".to_string(),
                updated_tb.unwrap(),
            );
        }

        Responses::panic("No user logged in".to_string(), json!({}))
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
            return Responses::success("TimeBlock deleted successfully".to_string(), json!({}));
        }

        Responses::panic("No user logged in".to_string(), json!({}))
    }

    async fn get_time_bock(&self, args: ShowTimeBlockArgs) -> Responses {
        let state = self.state.read().await;
        let mut tb = state.active_time_blocks.get(&args.name);

        if let Some(user) = state.user.as_ref() {
            if tb.is_none() {
                tb = user.blocks.get(&args.name);
            }

            return match tb {
                Some(tb) => Responses::success("Success".to_string(), tb),
                None => Responses::error("Time block not found".to_string(), json!({})),
            };
        }

        Responses::panic("No user logged in".to_string(), json!({}))
    }

    async fn list_all_time_blocks(&self) -> Responses {
        let state = self.state.read().await;

        if let Some(user) = state.user.as_ref() {
            let mut blocks: std::collections::HashMap<String, TimeBlock> = user.blocks.clone();
            blocks.extend(state.active_time_blocks.clone());

            let list = blocks.iter().map(|(_, tb)| tb).collect::<Vec<_>>();
            return Responses::success("Success".to_string(), list);
        }

        Responses::panic("No user logged in".to_string(), json!({}))
    }

    async fn list_active_time_blocks(&self) -> Responses {
        let state = self.state.read().await;
        if state.user.is_none() {
            return Responses::panic("No user logged in".to_string(), json!({}));
        }

        let list = state
            .active_time_blocks
            .iter()
            .map(|(_, tb)| tb)
            .filter(|tb| tb.state == StateBlock::InProgress)
            .collect::<Vec<_>>();
        Responses::success("Success".to_string(), list)
    }

    async fn start_time_block(&self, args: StartTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;
        let mut timeblock = None;

        if let Some(tb) = state.active_time_blocks.get(&args.name) {
            if tb.state == StateBlock::InProgress || tb.state == StateBlock::Paused {
                return Responses::error("Time block is already active".to_string(), json!({}));
            } else if tb.state == StateBlock::Idle {
                timeblock = Some(tb.clone());
            }
        }
        match state.user.as_ref() {
            Some(user) => match user.blocks.get(&args.name) {
                Some(tb) => {
                    timeblock = Some(tb.clone());
                }
                None => {
                    if timeblock.is_none() {
                        return Responses::error("Time block not found".to_string(), json!({}));
                    }
                }
            },
            None => {
                return Responses::panic("No user logged in".to_string(), json!({}));
            }
        }

        let mut timeblock = timeblock.unwrap();

        let now = Local::now();

        let duration = Duration::hours(timeblock.duration.get_local().hour() as i64)
            + Duration::minutes(timeblock.duration.get_local().minute() as i64);

        timeblock.start_time = TimeRegister::from_local(now.clone());
        timeblock.end_time = TimeRegister::from_local(now + duration);
        timeblock.state = StateBlock::InProgress;

        state.active_time_blocks.insert(args.name, timeblock);
        return Responses::success("Success".to_string(), json!({}));
    }

    async fn toggle_pause_time_block(&self, args: PauseTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;

        if state.user.is_none() {
            return Responses::panic("No user logged in".to_string(), json!({}));
        } else if !state.user.as_ref().unwrap().blocks.contains_key(&args.name) {
            return Responses::error("Time block not found".to_string(), json!({}));
        }

        if let Some(tb) = state.active_time_blocks.get_mut(&args.name) {
            match &tb.state {
                StateBlock::InProgress => {
                    tb.state = StateBlock::Paused;
                }

                StateBlock::Paused => {
                    tb.state = StateBlock::InProgress;
                }

                _ => {
                    return Responses::error(
                        "Time Block is not in a valid state".to_string(),
                        json!({}),
                    );
                }
            }

            return Responses::success("Success".to_string(), json!({}));
        } else {
            return Responses::error("Time Block is not activated".to_string(), json!({}));
        }
    }

    async fn stop_time_block(&self, args: StopTimeBlockArgs) -> Responses {
        let mut state = self.state.write().await;

        if state.user.is_none() {
            return Responses::panic("No user logged in".to_string(), json!({}));
        } else if !state.user.as_ref().unwrap().blocks.contains_key(&args.name) {
            return Responses::error("Time block not found".to_string(), json!({}));
        }

        if let Some(tb) = state.active_time_blocks.get_mut(&args.name) {
            match &tb.state {
                StateBlock::InProgress => {
                    tb.state = StateBlock::Finished;
                }

                StateBlock::Paused => {
                    tb.state = StateBlock::Finished;
                }

                _ => {
                    return Responses::error(
                        "Time Block is not in a valid state".to_string(),
                        json!({}),
                    );
                }
            }

            return Responses::success("Success".to_string(), json!({}));
        } else {
            return Responses::error("Time Block is not activated".to_string(), json!({}));
        }
    }

    async fn get_cofig(&self) -> Responses {
        let state = self.state.read().await;

        if let Some(user) = state.user.as_ref() {
            let config = &user.config;
            return Responses::success("Success".to_string(), config);
        }

        Responses::panic("No user logged in".to_string(), json!({}))
    }

    async fn update_cofig(&self, args: UpdateConfigArgs) -> Responses {
        let mut state = self.state.write().await;
        let storage = self.storage.clone();

        if let Some(user) = state.user.as_mut() {
            user.config.default_message = args.default_message;
            user.config.default_denied_acess = args.default_denied_acess;
            user.config.default_denied_apps = args.default_denied_apps;

            let _ = storage.save(user).await;
            return Responses::success("Success".to_string(), &user.config);
        }

        Responses::panic("No user logged in".to_string(), json!({}))
    }
}

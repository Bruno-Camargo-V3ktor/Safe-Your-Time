use super::{BuildService, Service, ServicePool};
use crate::{
    managers::SharedManager,
    models::{StateBlock, TimeRegister},
    state_app::SharedStateApp,
    utils::{start_timeblock_notification, stop_timeblock_notification},
};
use chrono::{Datelike, Local, Timelike};

pub struct TimerService {
    state: SharedStateApp,
    manager: SharedManager,
}

pub struct BuildTimerService;

#[async_trait::async_trait]
impl BuildService for BuildTimerService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = TimerService::new(
            states.get_state::<SharedStateApp>().await.unwrap(),
            states.get_state::<SharedManager>().await.unwrap(),
        );

        Box::new(service)
    }
}

impl TimerService {
    pub fn build() -> BuildTimerService {
        BuildTimerService {}
    }

    pub fn new(state: SharedStateApp, manager: SharedManager) -> Self {
        Self { state, manager }
    }

    pub fn send_notifications(
        &self,
        initiated: Vec<(String, Option<String>)>,
        finished: Vec<String>,
        default_message: String,
    ) {
        for (name, message) in initiated {
            let message = message.unwrap_or(default_message.clone());
            start_timeblock_notification(&self.manager, name, message);
        }

        for name in finished {
            stop_timeblock_notification(&self.manager, name);
        }
    }
}

#[async_trait::async_trait]
impl Service for TimerService {
    async fn exec(&mut self) {
        let now_time = Local::now();
        let weekday = now_time.weekday();
        let actual_time = TimeRegister::new(now_time.hour(), now_time.minute()).unwrap();

        let mut initiated = vec![];
        let mut finished = vec![];

        let mut state = self.state.write().await;
        if state.user.is_none() {
            return;
        }

        state.active_time_blocks.retain(|_, tb| {
            if tb.end_time < actual_time {
                if tb.state != StateBlock::Finished {
                    finished.push(tb.name.clone());
                }
                return false;
            }

            let entry_time = tb.start_time <= actual_time && tb.end_time >= actual_time;
            if tb.state == StateBlock::Idle && entry_time {
                tb.state = StateBlock::InProgress;
                initiated.push((tb.name.clone(), tb.message.clone()));
            }

            true
        });

        let user = state.user.as_ref().unwrap();
        let default_message = user.config.default_message.clone();

        let time_blocks_for_day = user
            .blocks
            .iter()
            .filter(|(_, tb)| tb.days.contains(&weekday))
            .map(|(_, tb)| tb.clone())
            .collect::<Vec<_>>();

        for mut tb in time_blocks_for_day {
            if tb.start_time <= actual_time && tb.end_time >= actual_time {
                if state.active_time_blocks.get(&tb.name).is_none() {
                    tb.state = StateBlock::InProgress;
                    initiated.push((tb.name.clone(), tb.message.clone()));
                    state.active_time_blocks.insert(tb.name.clone(), tb);
                }
            }
        }

        self.send_notifications(initiated, finished, default_message);
    }
}

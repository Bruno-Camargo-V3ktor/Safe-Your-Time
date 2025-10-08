use super::{ Service, BuildService, ServicePool };
use crate::{ models::{ StateBlock, TimeRegister }, state_app::SharedStateApp };
use chrono::{ Datelike, Local, Timelike };

pub struct TimerService {
    state: SharedStateApp,
}

pub struct BuildTimerService;

#[async_trait::async_trait]
impl BuildService for BuildTimerService {
    async fn build(&self, states: &ServicePool) -> Box<dyn Service + Send + Sync> {
        let service = TimerService::new(states.get_state::<SharedStateApp>().await.unwrap());

        Box::new(service)
    }
}

impl TimerService {
    pub fn build() -> BuildTimerService {
        BuildTimerService {}
    }

    pub fn new(state: SharedStateApp) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for TimerService {
    async fn exec(&mut self) {
        let now_time = Local::now();
        let weekday = now_time.weekday();
        let actual_time = TimeRegister::new(now_time.hour(), now_time.minute()).unwrap();

        let mut state = self.state.write().await;
        if state.user.is_none() {
            return;
        }

        state.active_time_blocks.retain(|_, tb| {
            if tb.end_time < actual_time {
                return false;
            }

            let entry_time = tb.start_time <= actual_time && tb.end_time >= actual_time;
            if tb.state == StateBlock::Idle && entry_time {
                tb.state = StateBlock::InProgress;
            }

            true
        });

        let user = state.user.as_ref().unwrap();
        let time_blocks_for_day = user.blocks
            .iter()
            .filter(|(_, tb)| tb.days.contains(&weekday))
            .map(|(_, tb)| tb.clone())
            .collect::<Vec<_>>();

        for mut tb in time_blocks_for_day {
            if tb.start_time <= actual_time && tb.end_time >= actual_time {
                if state.active_time_blocks.get(&tb.name).is_none() {
                    tb.state = StateBlock::InProgress;
                    state.active_time_blocks.insert(tb.name.clone(), tb);
                }
            }
        }
    }
}

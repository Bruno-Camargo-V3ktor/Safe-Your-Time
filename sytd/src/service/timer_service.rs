use super::Service;
use crate::{ models::{ StateBlock, TimeRegister }, state_app::SharedStateApp };
use chrono::{ Datelike, Local, Timelike };

pub struct TimerService {
    state: SharedStateApp,
}

impl TimerService {
    pub fn new(state: SharedStateApp) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl Service for TimerService {
    async fn exec(&mut self) {
        let now_time = Local::now();
        let weekday = now_time.weekday();
        let actual_time = TimeRegister::new(
            now_time.hour() as u8,
            now_time.minute() as u8
        ).unwrap();

        let mut state = self.state.write().await;
        if state.user.is_none() {
            return;
        }

        state.active_time_blocks.retain(|_, tb| {
            if tb.end_time < actual_time {
                return false;
            }

            if
                tb.state == StateBlock::Idle &&
                tb.start_time <= actual_time &&
                tb.end_time >= actual_time
            {
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

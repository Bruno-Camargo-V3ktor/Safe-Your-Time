use chrono::{Datelike, Local, Timelike};

use crate::{models::TimeRegister, state_app::SharedStateApp};

use super::Service;

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
        let actual_time =
            TimeRegister::new(now_time.hour() as u8, now_time.minute() as u8).unwrap();

        let mut state = self.state.write().await;
        if let Some(timeblock) = &state.active_time_block {
            if actual_time >= timeblock.end_time {
                state.active_time_block = None;
            }
        }
    }
}

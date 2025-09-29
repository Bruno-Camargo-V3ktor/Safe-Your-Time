use std::collections::HashMap;

use super::Service;
use crate::{models::TimeRegister, state_app::SharedStateApp};
use chrono::{Datelike, Local, Timelike};

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

        if state.user.is_none() {
            return;
        }

        let time_blocks_for_day = state
            .active_time_blocks
            .iter()
            .filter(|(_, tb)| tb.days.contains(&weekday))
            .map(|(_, tb)| tb.clone())
            .collect::<Vec<_>>();

        state.active_time_blocks.retain(|_, tb| {
            return true;
        });
    }
}

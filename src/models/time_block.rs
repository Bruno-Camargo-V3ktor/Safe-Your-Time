use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeBlock {
    pub durantion: Duration,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,

    pub denied_acess: Vec<String>,
    pub allow_acess: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,
}

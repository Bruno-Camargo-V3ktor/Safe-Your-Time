use super::{DayOfWeek, StateBlock, TimeRegister};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeBlock {
    pub name: String,
    pub message: Option<String>,
    pub duration: TimeRegister,
    pub start_time: TimeRegister,
    pub end_time: TimeRegister,
    pub state: StateBlock,

    pub days: HashSet<DayOfWeek>,

    pub denied_acess: Vec<String>,
    pub allow_acess: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,
}

use chrono::Weekday;
use serde::{Deserialize, Serialize};

use syt_models::TimeRegister;

pub async fn from_bytes(bytes: &[u8]) -> anyhow::Result<Commands> {
    let command: Commands = serde_json::from_slice(bytes)?;
    Ok(command)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "args")]
pub enum Commands {
    CreateTimeBlock(CreateTimeBlockArgs),
    UpdateTimeBlock(UpdateTimeBlockArgs),
    DeleteTimeBlock(DeleteTimeBlockArgs),
    ShowTimeBlock(ShowTimeBlockArgs),

    ListTimeBlocks,
    ShowActiveTimeBlocks,

    StartTimeBlock(StartTimeBlockArgs),
    PauseTimeBlock(PauseTimeBlockArgs),
    StopTimeBlock(StopTimeBlockArgs),

    ShowConfig,
    UpdateConfig(UpdateConfigArgs),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTimeBlockArgs {
    pub name: String,

    pub duration: Option<TimeRegister>,
    pub start_time: Option<TimeRegister>,
    pub end_time: Option<TimeRegister>,

    pub message: Option<String>,

    pub denied_web: Vec<String>,
    pub allow_web: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,

    pub days: Vec<Weekday>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTimeBlockArgs {
    pub name: String,
    pub new_name: Option<String>,

    pub duration: Option<TimeRegister>,
    pub start_time: Option<TimeRegister>,
    pub end_time: Option<TimeRegister>,

    pub message: Option<String>,

    pub denied_web: Vec<String>,
    pub allow_web: Vec<String>,

    pub denied_apps: Vec<String>,
    pub allow_apps: Vec<String>,

    pub days: Vec<Weekday>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTimeBlockArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTimeBlockArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauseTimeBlockArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTimeBlockArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowTimeBlockArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    pub default_denied_acess: Vec<String>,
    pub default_denied_apps: Vec<String>,
    pub default_message: String,
}

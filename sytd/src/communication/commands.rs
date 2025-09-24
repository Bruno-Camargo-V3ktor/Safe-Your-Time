use chrono::{ DateTime, Duration, Local };
use serde::{ Deserialize, Serialize };

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
    StatusTimeBlock,
    StartTimeBlock(StartTimeBlockArgs),
    StopTimeBlock(StopTimeBlockArgs),
    ShowTimeBlock(ShowTimeBlockArgs),
    ListTimeBlocks,
    ShowConfig,
    UpdateConfig(UpdateConfigArgs),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTimeBlockArgs {
    name: String,

    duration: Option<Duration>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,

    message: Option<String>,

    denied_web: Option<Vec<String>>,
    allow_web: Option<Vec<String>>,

    denied_apps: Option<Vec<String>>,
    allow_apps: Option<Vec<String>>,

    days: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTimeBlockArgs {
    name: String,

    new_name: Option<String>,

    duration: Option<Duration>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,

    message: Option<String>,

    denied_web: Option<Vec<String>>,
    allow_web: Option<Vec<String>>,

    denied_apps: Option<Vec<String>>,
    allow_apps: Option<Vec<String>>,

    days: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTimeBlockArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTimeBlockArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTimeBlockArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowTimeBlockArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    pub system_apps: Option<Vec<String>>,
    pub default_denied_acess: Option<Vec<String>>,
    pub default_denied_apps: Option<Vec<String>>,
    pub default_message: Option<String>,
    pub http_listening: Option<bool>,
}

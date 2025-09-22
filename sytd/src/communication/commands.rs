use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

pub async fn from_bytes(bytes: &[u8]) -> anyhow::Result<Commands> {
    let command: Commands = serde_json::from_slice(bytes)?;
    Ok(command)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "args")]
pub enum Commands {
    Create(CreateArgs),
    Update(UpdateArgs),
    Delete(DeleteArgs),
    Status,
    Start(StartArgs),
    Stop(StopArgs),
    Show(ShowArgs),
    List,
    Config(ConfigArgs),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArgs {
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
pub struct UpdateArgs {
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
pub struct DeleteArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigArgs {}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub system_apps: Vec<String>,
    pub monitoring_time: u16,
    pub reload_time: u16,
    pub default_denied_acess: Vec<String>,
    pub default_denied_apps: Vec<String>,
    pub default_message: String,
}

impl AppConfig {
    pub fn default_configs() -> Self {
        Self {
            system_apps: vec![],
            monitoring_time: 4000,
            reload_time: 5000,
            default_denied_acess: vec![],
            default_denied_apps: vec![],
            default_message: String::from("Focus!!!"),
        }
    }
}

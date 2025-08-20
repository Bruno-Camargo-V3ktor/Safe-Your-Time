use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub system_apps: Vec<String>,
    pub monitoring_time_secs: u8,
    pub reload_time_secs: u8,
    pub default_denied_acess: Vec<String>,
    pub default_denied_apps: Vec<String>,
    pub default_redirect: String,
}

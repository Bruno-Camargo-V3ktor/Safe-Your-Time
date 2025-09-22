use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub system_apps: Vec<String>,
    pub default_denied_acess: Vec<String>,
    pub default_denied_apps: Vec<String>,
    pub default_message: String,
    pub http_listening: bool,
    pub firewall_block: bool,
}

impl AppConfig {
    pub fn default_configs() -> Self {
        Self {
            system_apps: vec![],
            default_denied_acess: vec![],
            default_denied_apps: vec![],
            default_message: String::from("Focus!!!"),
            http_listening: true,
            firewall_block: false,
        }
    }
}

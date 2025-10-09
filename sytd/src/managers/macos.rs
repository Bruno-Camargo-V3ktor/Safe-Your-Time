use super::Manager;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

use crate::utils::get_dir;
pub struct MacOsManager {}

impl Manager for MacOsManager {
    async fn monitoring_apps(&self, _apps: Vec<String>) {
        todo!()
    }

    async fn kill_process(&self, _id_process: String) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_username(&self) -> anyhow::Result<String> {
        todo!()
    }

    async fn notification(&self, title: String, body: String) -> anyhow::Result<()> {
        let current_dir = get_dir();
        let icon_path = PathBuf::from(current_dir)
            .join("imgs/warning.png")
            .to_string_lossy()
            .to_string();

        let _ = tokio::task::spawn_blocking(move || {
            Notification::new()
                .summary(&title)
                .body(&body)
                .icon(&icon_path)
                .urgency(Urgency::Critical)
                .show()
        })
        .await??;

        Ok(())
    }
}

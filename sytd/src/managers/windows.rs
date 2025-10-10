use std::{ path::PathBuf, process::Stdio };

use anyhow::anyhow;
use notify_rust::Notification;
use tokio::process::Command;

use crate::utils::get_dir;

use super::Manager;

pub struct WindowsManager {}

#[async_trait::async_trait]
impl Manager for WindowsManager {
    async fn monitoring_apps(&self, apps: Vec<String>) {
        let output = Command::new("tasklist").output().await.unwrap();
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    let app_name = parts[0]
                        .to_string()
                        .to_lowercase()
                        .strip_suffix(".exe")
                        .unwrap_or_default()
                        .to_string();

                    if apps.contains(&app_name) {
                        let pid = parts[1];
                        if let Ok(pid) = pid.parse::<i32>() {
                            let res = self.kill_process(pid.to_string()).await;
                            if let Err(e) = res {
                                eprintln!("{e}");
                            }
                        }
                    }
                }
            }
        }
    }

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()> {
        Command::new("taskkill")
            .args(["/PID", &id_process, "/F"])
            .stdout(std::process::Stdio::null())
            .status().await?;

        Ok(())
    }

    async fn get_username(&self) -> anyhow::Result<String> {
        let output = Command::new("whoami")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output().await?;

        if output.status.success() {
            let username = String::from_utf8(output.stdout)?.trim().to_string();
            Ok(username)
        } else {
            let error_message = String::from_utf8(output.stderr)?;
            Err(anyhow!(error_message))
        }
    }

    fn notification(
        &self,
        title: String,
        subtitle: String,
        body: String,
        icon: String
    ) -> anyhow::Result<()> {
        let current_dir = get_dir();
        let icon_path = PathBuf::from(current_dir)
            .join(format!("imgs/{}", icon))
            .to_string_lossy()
            .to_string();

        let _ = tokio::task::spawn_blocking(move || {
            Notification::new()
                .summary(&title)
                .subtitle(&subtitle)
                .body(&body)
                .icon(&icon_path)
                .appname("Safe your Time")
                .show()
        });

        Ok(())
    }
}

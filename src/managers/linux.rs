use tokio::process::Command;

use super::Manager;

pub struct LinuxManager {}

impl Manager for LinuxManager {
    async fn monitoring_apps(&self, apps: Vec<String>) {
        let output = Command::new("top")
            .args(["-b", "-n", "1"])
            .output()
            .await
            .unwrap();

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for file in stdout.lines().skip(7) {
                let parts: Vec<&str> = file.split_whitespace().collect();
                let app_name = parts.last().unwrap().to_string().to_lowercase();
                let pid = parts[0].to_string();

                if apps.contains(&app_name) {
                    let res = self.kill_process(pid).await;
                    if let Err(e) = res {
                        eprintln!("{e}");
                    }
                }
            }
        }
    }

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()> {
        Command::new("kill")
            .args(["-9", &id_process])
            .status()
            .await?;

        Ok(())
    }

    async fn get_username(&self) -> anyhow::Result<String> {
        todo!()
    }

    async fn notification(&self) -> anyhow::Result<()> {
        todo!()
    }
}

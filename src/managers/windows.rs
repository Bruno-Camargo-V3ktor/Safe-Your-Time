use tokio::process::Command;

use super::Manager;

pub struct WindowsManager {}

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
            .status()
            .await?;

        Ok(())
    }

    async fn firewall_block(
        &self,
        ip_block: String,
        ip_redirect: String,
        rule_name: String,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn firewall_allow(&self, ip: String, rule_name: String) -> anyhow::Result<()> {
        todo!()
    }

    async fn firewall_clean(&self, rule_name: String) -> anyhow::Result<()> {
        todo!()
    }

    async fn domain_resolve(&self, domain: String) -> String {
        todo!()
    }
}

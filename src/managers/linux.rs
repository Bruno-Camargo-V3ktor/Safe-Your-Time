use super::Manager;

pub struct LinuxManager {}

impl Manager for LinuxManager {
    fn monitoring_apps(&self, apps: Vec<String>) {
        todo!()
    }

    fn kill_process(&self, id_process: String) -> anyhow::Result<()> {
        todo!()
    }

    fn firewall_block(
        &self,
        ip_block: String,
        ip_redirect: String,
        rule_name: String,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn firewall_allow(&self, ip: String, rule_name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn firewall_clean(&self, rule_name: String) -> anyhow::Result<()> {
        todo!()
    }
}

use super::Manager;

pub struct MacOsManager {}

impl Manager for MacOsManager {
    async fn monitoring_apps(&self, apps: Vec<String>) {
        todo!()
    }

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()> {
        todo!()
    }

    async fn firewall_block(&self, ip: String, rule_name: String) -> anyhow::Result<()> {
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

use super::Manager;

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

    async fn notification(&self, _title: String, _body: String) -> anyhow::Result<()> {
        todo!()
    }
}

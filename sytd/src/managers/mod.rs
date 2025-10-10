use std::sync::Arc;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub type SharedManager = Arc<dyn Manager + Send + Sync>;

// Tratis
#[async_trait::async_trait]
pub trait Manager {
    async fn monitoring_apps(&self, apps: Vec<String>);

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()>;

    async fn get_username(&self) -> anyhow::Result<String>;

    fn notification(
        &self,
        title: String,
        subtitle: String,
        body: String,
        icon: String
    ) -> anyhow::Result<()>;
}

#[cfg(target_os = "windows")]
pub fn get_manager() -> SharedManager {
    use std::sync::Arc;
    use windows::WindowsManager;

    Arc::new(WindowsManager {})
}

#[cfg(target_os = "linux")]
pub fn get_manager() -> SharedManager {
    use linux::LinuxManager;
    use std::sync::Arc;

    Arc::new(LinuxManager {})
}

#[cfg(target_os = "macos")]
pub fn get_manager() -> SharedManager {
    use macos::MacOsManager;
    use std::sync::Arc;

    Arc::new(MacOsManager {})
}

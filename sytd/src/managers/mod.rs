#[cfg(target_os = "linux")]
mod linux;
mod macos;
mod windows;

// Tratis
pub trait Manager {
    async fn monitoring_apps(&self, apps: Vec<String>);

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()>;

    async fn get_username(&self) -> anyhow::Result<String>;

    async fn notification(&self, title: String, body: String) -> anyhow::Result<()>;
}

#[cfg(target_os = "windows")]
pub fn get_manager() -> windows::WindowsManager {
    use windows::WindowsManager;
    WindowsManager {}
}

#[cfg(target_os = "linux")]
pub fn get_manager() -> linux::LinuxManager {
    use linux::LinuxManager;
    LinuxManager {}
}

#[cfg(target_os = "macos")]
pub fn get_manager() -> macos::MacOsManager {
    use macos::MacOsManager;
    MacOsManager {}
}

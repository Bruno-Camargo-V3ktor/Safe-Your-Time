#[cfg(target_os = "linux")]
mod linux;
mod macos;
mod windows;

// Tratis
pub trait Manager {
    async fn monitoring_apps(&self, apps: Vec<String>);

    async fn kill_process(&self, id_process: String) -> anyhow::Result<()>;

    async fn domain_resolve(&self, domain: String) -> String;

    async fn firewall_block(&self, ip: String, rule_name: String) -> anyhow::Result<()>;

    async fn firewall_allow(&self, ip: String, rule_name: String) -> anyhow::Result<()>;

    async fn firewall_clean(&self, rule_name: String) -> anyhow::Result<()>;
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

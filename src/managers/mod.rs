mod linux;
mod macos;
mod windows;

// Tratis
pub trait Manager {
    fn monitoring_apps(&self, apps: Vec<String>);

    fn kill_process(&self, id_process: String) -> anyhow::Result<()>;

    fn firewall_block(
        &self,
        ip_block: String,
        ip_redirect: String,
        rule_name: String,
    ) -> anyhow::Result<()>;

    fn firewall_allow(&self, ip: String, rule_name: String) -> anyhow::Result<()>;

    fn firewall_clean(&self, rule_name: String) -> anyhow::Result<()>;
}

#[cfg(target_os = "windows")]
pub fn get_manager() -> impl Manager {
    use windows::WindowsManager;
    WindowsManager {}
}

#[cfg(target_os = "linux")]
pub fn get_manager() -> impl Manager {
    use linux::LinuxManager;
    LinuxManager {}
}

#[cfg(target_os = "macos")]
pub fn get_manager() -> impl Manager {
    use macos::MacOsManager;
    MacOsManager {}
}

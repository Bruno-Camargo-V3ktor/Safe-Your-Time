mod commands;
mod controller;
mod listener_socket;
mod responses;

pub use commands::*;
pub use controller::*;
pub use listener_socket::*;
pub use responses::*;

#[async_trait::async_trait]
pub trait Listener {
    fn get_controller(&self) -> SharedController;
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()>;
}

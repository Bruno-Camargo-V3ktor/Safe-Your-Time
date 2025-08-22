mod commands;
mod controller;
mod listener;
mod responses;

pub use commands::*;
pub use controller::*;
pub use listener::*;
pub use responses::*;

#[async_trait::async_trait]
pub trait Listener {
    fn get_controller(&self) -> &Controller;
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()>;
}

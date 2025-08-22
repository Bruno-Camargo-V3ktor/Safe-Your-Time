mod commands;
mod listener;

pub use listener::*;

#[async_trait::async_trait]
pub trait Listener {
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()>;
}

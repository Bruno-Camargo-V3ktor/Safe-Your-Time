mod listener_socket;
mod listener_http;
mod controller;
mod routers;

pub use listener_socket::*;
pub use controller::*;
pub use listener_http::*;

#[async_trait::async_trait]
pub trait Listener {
    fn get_controller(&self) -> SharedController;
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()>;
}

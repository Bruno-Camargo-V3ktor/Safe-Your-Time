use super::Listener;
use super::commands;
use super::controller::Controller;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct ListenerSockter {
    controller: Controller,
}

impl ListenerSockter {
    pub fn new(controller: Controller) -> Self {
        Self { controller }
    }
}

#[async_trait::async_trait]
impl Listener for ListenerSockter {
    fn get_controller(&self) -> &Controller {
        &self.controller
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()> {
        use tokio::net::UnixListener;

        let controller = self.get_controller();
        let socket_path = addr.into();
        let _ = tokio::fs::remove_file(socket_path.clone()).await;

        let listener_local = UnixListener::bind(socket_path.clone())?;
        loop {
            let (mut socket, _addr) = listener_local.accept().await?;

            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let command = commands::from_bytes(&buf[..n]).await.unwrap();
            let response = controller.process(command).await;

            let _ = socket.write_all(&response.to_bytes()).await;
        }
    }

    #[cfg(target_os = "windows")]
    async fn server(&self, _addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let socket_path = r"\\.\pipe\sytd".to_string();
        let controller = self.get_controller();

        loop {
            let mut pipe = ServerOptions::new()
                .first_pipe_instance(false)
                .create(&socket_path)
                .unwrap();

            pipe.connect().await.unwrap();

            let mut buf = vec![0; 1024];
            if let Ok(n) = pipe.read(&mut buf).await {
                let command = commands::from_bytes(&buf[..n]).await.unwrap();
                let response = controller.process(command).await;
                let _ = pipe.write_all(&response.to_bytes()).await;
            }
        }
    }
}

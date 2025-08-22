use crate::communication::Listener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct ListenerSockter {}

impl ListenerSockter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Listener for ListenerSockter {
    #[cfg(target_os = "linux")]
    async fn server(&self, addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()> {
        use tokio::net::UnixListener;

        let socket_path = addr.into();
        let _ = tokio::fs::remove_file(socket_path.clone()).await;

        let listener_local = UnixListener::bind(socket_path.clone())?;
        loop {
            let (mut socket, _addr) = listener_local.accept().await?;

            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let received = String::from_utf8_lossy(&buf[..n]);
            println!("Recevid: {received}");

            let _ = socket.write_all(b"Hi").await;
        }
    }

    #[cfg(target_os = "windows")]
    async fn server(&self, _addr: impl Into<String> + std::marker::Send) -> anyhow::Result<()> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let socket_path = r"\\.\pipe\sytd".to_string();
        let mut listener_local = ServerOptions::new()
            .first_pipe_instance(true)
            .create(socket_path.clone())
            .unwrap();

        loop {
            let _ = listener_local.connect().await.unwrap();
            let mut buf = vec![0; 1024];

            let n = listener_local.read(&mut buf).await.unwrap();
            let received = String::from_utf8_lossy(&buf[..n]);
            println!("Recevid: {received}");

            let _ = listener_local.write_all(b"Hi").await;
        }
    }
}

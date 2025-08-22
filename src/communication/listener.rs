use crate::communication::Listener;
use tokio::fs;

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
        let _ = fs::remove_file(socket_path.clone()).await;

        let listener_local = UnixListener::bind(socket_path.clone())?;
        loop {
            let (mut socket, _addr) = listener_local.accept().await?;

            tokio::spawn(async move {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                let mut buf = vec![0; 1024];
                let n = socket.read(&mut buf).await.unwrap();
                let received = String::from_utf8_lossy(&buf[..n]);
                println!("Recevid: {received}");

                let _ = socket.write_all(b"Hi").await;
            });
        }
    }
}

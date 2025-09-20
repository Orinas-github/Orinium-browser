use tokio::net::TcpStream;
use tokio::io::{AsyncRead, AsyncWrite};
use std::time::Duration;

#[derive(Debug)]
pub struct TcpConnection {
    pub stream: TcpStream,
}

impl TcpConnection {
    /// TCP接続を作成、指定したタイムアウト時間内に接続できなければエラーを返す
    pub async fn connect(host: &str, port: u16, timeout: Duration) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", host, port);
        let stream = tokio::time::timeout(timeout, TcpStream::connect(addr)).await??;
        Ok(Self { stream })
    }
}

impl AsyncRead for TcpConnection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for TcpConnection {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_shutdown(cx)
    }
}


use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use rustls::ClientConfig;
use rustls::pki_types::ServerName;
use rustls_native_certs::load_native_certs;
use tokio_rustls::client::TlsStream;
use tokio_rustls::TlsConnector;
use std::time::Duration;

use crate::platform::network::tcp::TcpConnection;

#[derive(Debug)]
pub struct TlsConnection {
    pub stream: TlsStream<TcpConnection>,
}

impl TlsConnection {
    /// TLS接続を作成、指定したタイムアウト時間内に接続できなければエラーを返す
    pub async fn connect(host: &str, port: u16, timeout: Duration) -> anyhow::Result<Self> {
        // TCP接続を確立
        let tcp_conn = TcpConnection::connect(host, port, timeout).await?;

        // 証明書ストアから証明書を読み込む
        let mut roots = rustls::RootCertStore::empty();
        for cert in load_native_certs()? {
            roots.add(cert)?;
        }

        // TLS設定
        let config = ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();

        // TLSコネクタを作成
        let connector = TlsConnector::from(Arc::new(config));

        // ホスト名をDNS名に変換
        let server_name = ServerName::try_from(host.to_string())
            .map_err(|_| anyhow::anyhow!("Invalid DNS name: {}", host))?;

        let stream = tokio::time::timeout(
            timeout,
            connector.connect(server_name, tcp_conn)
        ).await??;

        Ok(Self { stream })
    }
}

impl AsyncRead for TlsConnection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for TlsConnection {
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

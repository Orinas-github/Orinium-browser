use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

/// TCP接続を管理する構造体
///
/// この構造体は基本的なTCP接続とデータ送受信機能を提供します。
#[derive(Debug)]
pub struct TcpConnection {
    /// 内部のTCPストリーム
    pub stream: TcpStream,
}

impl TcpConnection {
    /// TCP接続を作成します。
    ///
    /// # 引数
    /// * `host` - 接続先のホスト名またはIPアドレス
    /// * `port` - 接続先のポート番号
    /// * `timeout` - 接続タイムアウト時間
    ///
    /// # 戻り値
    /// * 成功した場合は`TcpConnection`のインスタンスを返します
    /// * タイムアウトまたは接続エラーの場合は`anyhow::Error`を返します
    pub async fn connect(host: &str, port: u16, timeout: Duration) -> anyhow::Result<Self> {
        let addr = format!("{host}:{port}");
        let stream = tokio::time::timeout(timeout, TcpStream::connect(addr)).await??;
        Ok(Self { stream })
    }
}

/// `AsyncRead`トレイトの実装により、TcpConnectionから非同期的にデータを読み取る機能を提供します。
impl AsyncRead for TcpConnection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

/// `AsyncWrite`トレイトの実装により、TcpConnectionに非同期的にデータを書き込む機能を提供します。
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

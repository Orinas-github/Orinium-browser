use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::platform::network::tcp::TcpConnection;
// use crate::platform::network::tls::TlsConnection;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct HostKey {
    pub scheme: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub enum Connection {
    Tcp(TcpConnection),
    //    Tls(TlsConnection),
}

#[derive(Debug)]
pub struct ConnectionPool {
    pool: Arc<RwLock<HashMap<HostKey, Vec<Connection>>>>,
    pub max_connections_per_host: usize,
}

impl ConnectionPool {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(RwLock::new(HashMap::new())),
            max_connections_per_host: 6,
        }
    }

    pub async fn get_connection(&self, key: &HostKey) -> Option<Connection> {
        let mut pool = self.pool.write().await;
        pool.get_mut(key).and_then(|vec| vec.pop())
    }

    pub async fn add_connection(&self, key: HostKey, conn: Connection) {
        let mut pool = self.pool.write().await;
        let entry = pool.entry(key).or_insert_with(Vec::new);
        if entry.len() < self.max_connections_per_host {
            entry.push(conn);
        }
    }

    #[allow(dead_code)]
    pub async fn close_all(&self) {
        let mut pool = self.pool.write().await;
        pool.clear();
    }
}

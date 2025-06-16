use crate::connection::id;
use crate::protocol;
use dashmap::DashMap;
use id::ConnectionId;
use protocol::ProtocolType;
use std::sync::Arc;
use tokio::sync::oneshot;

#[allow(dead_code)]
pub struct ConnectionManager {
    inner: Arc<ConnectionManagerState>,
}

#[allow(dead_code)]
struct ConnectionManagerState {
    connections: DashMap<ConnectionId, ActiveConnection>,
}

#[allow(dead_code)]
struct ActiveConnection {
    pub id: ConnectionId,
    pub peer_addr: std::net::SocketAddr,
    pub protocol: ProtocolType,
    pub created_at: std::time::Instant,
    pub close_tx: oneshot::Sender<()>,
}

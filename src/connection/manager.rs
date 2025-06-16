
use std::sync::Arc;
use dashmap::DashMap;
use id::ConnectionId;
use crate::connection::id;
use protocol::ProtocolType;
use crate::protocol;
use tokio::sync::{mpsc, oneshot};

pub struct ConnectionManager {
    inner: Arc<ConnectionManagerState>
}

struct ConnectionManagerState {
    connections: DashMap<ConnectionId, ActiveConnection>
}

struct ActiveConnection {
    pub id: ConnectionId,
    pub peer_addr: std::net::SocketAddr,
    pub protocol: ProtocolType,
    pub created_at: std::time::Instant,
    pub close_tx: oneshot::Sender<()>
}

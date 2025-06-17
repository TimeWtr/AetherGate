use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tokio::task::JoinHandle;

mod quic;
mod tcp;
mod udp_kcp;

#[allow(dead_code)]
trait AsyncIOEngine: Send + Sync + 'static {
    // create network listener.
    async fn bind(&self, addr: &str, conf: &IOConfig) -> Result<Box<dyn AsyncListener>, ()>;
    // start event loop.
    fn run_event_loop(&self, shutdown: Arc<AtomicBool>) -> JoinHandle<()>;
}

#[allow(dead_code)]
pub struct IOConfig {
    pub max_connections: usize,
    pub send_buffer_size: usize,
    pub recv_buffer_size: usize,
    pub tcp_nodelay: bool,
    pub tcp_keepalive_interval: Option<Duration>,
    pub tcp_keepalive_timeout: Option<Duration>,
    pub quic_0rtt: bool,
    pub kcp_fast_mode: bool,
}

#[allow(dead_code)]
#[async_trait]
pub trait AsyncListener {
    // accept new connection.
    async fn accept(&self) -> Result<Box<dyn AsyncConnection>, SocketAddr>;
}

pub trait AsyncConnection: Send + Sync {}

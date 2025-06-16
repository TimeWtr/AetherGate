mod buffer_pool;
mod id;
mod manager;
mod quic;
mod stream;
mod tcp;
mod udp;
mod websocket;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ConnectionConfig {
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub write_buffer_size: usize,
    pub tcp_keepalive_secs: Option<u32>,
}

mod manager;
mod stream;
mod tcp;
mod udp;
mod quic;
mod websocket;
mod buffer_pool;
mod id;


pub struct ConnectionConfig {
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub write_buffer_size: usize,
    pub tcp_keepalive_secs: Option<u32>,
}

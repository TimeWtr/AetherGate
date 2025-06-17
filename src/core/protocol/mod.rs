mod adapter;
mod codec;
mod fb_codec;
mod pb_codec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ProtocolType {
    Tcp,
    Udp,
    Http,
    Websocket,
    Quic,
}

#[allow(dead_code)]
trait Protocol {
    fn send(&self, data: &[u8]) -> Result<(), String>;
    fn receive(&self) -> Result<Vec<&u8>, String>;
}

mod adapter;
mod codec;


pub enum ProtocolType {
    TCP,
    UDP,
    HTTP,
    WEBSOCKET,
    QUIC,
}
trait Protocol {
    fn Send(&self, data: &[u8]) -> Result<(), String>;
    fn Receive(&self) -> Result<Vec<&u8>, String>;
}

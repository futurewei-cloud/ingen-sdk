mod simple_tcp_server;

use anyhow::Result;
use bytes::Bytes;
use simple_tcp_server::*;

struct ServerSessionHandler {}

impl SessionHandler for ServerSessionHandler {
    fn handle_incoming_data(&mut self, incoming: &[u8]) -> Result<(usize, Bytes)> {
        let response = Bytes::copy_from_slice(incoming);
        Ok((incoming.len(), response))
    }
}

fn main() -> anyhow::Result<()> {
    let server_endpoint = "127.0.0.1:8888";
    SimpleTcpServer::new(
        &server_endpoint,
        Box::new(|| Box::new(ServerSessionHandler {})),
    )
    .run()
}

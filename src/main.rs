mod port;
mod serializer;

use crate::port::unix_port::{UnixSocketPort, UnixSocketSettings};
use crate::port::InputPort;
use std::io;

static SOCKET_ADDRESS: &str = "/var/run/dispatcher";

#[tokio::main]
async fn main() -> io::Result<()> {
    let port_settings = UnixSocketSettings {
        socket_path: String::from(SOCKET_ADDRESS),
    };
    let port = UnixSocketPort::new(port_settings);
    return port.receive().await;
}

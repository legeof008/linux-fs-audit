mod controller;
mod serializer;
mod view;
mod settings;

use crate::controller::unix_port::{UnixSocketPort, UnixSocketSettings};
use crate::controller::InputPort;
use crate::view::MockView;
use log::Level;

static SOCKET_ADDRESS: &str = "/var/run/dispatcher";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    let port_settings = UnixSocketSettings {
        socket_path: String::from(SOCKET_ADDRESS),
    };
    let port = UnixSocketPort::new(port_settings, Box::new(MockView {}));
    port.receive().await.expect("Fatal: Port cannot recive inputs.");
    Ok(())
}

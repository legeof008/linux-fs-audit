mod controller;
mod serializer;
mod settings;
mod view;

use crate::controller::unix_port::{UnixSocketPort, UnixSocketSettings};
use crate::controller::InputPort;
use crate::settings::{configure, ViewMode};
use crate::view::{HttpView, MockView, View};
use log::Level;

static SETTINGS_ADDRESS: &str = "./resources/settings.json";
static HTTP_VIEW_DESTINATION: &str = "localhost:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    log::debug!("Loading settings from: {}", SETTINGS_ADDRESS);
    let configs = configure(SETTINGS_ADDRESS)?;
    let view: Box<dyn View> = match configs.view_mode {
        ViewMode::Http => Box::new(HttpView::new(HTTP_VIEW_DESTINATION)),
        ViewMode::Mock => Box::new(MockView {}),
    };
    let port_settings = UnixSocketSettings {
        socket_path: configs.dispatcher_directory,
    };
    let port = UnixSocketPort::new(port_settings, view);
    port.receive()
        .await
        .expect("Fatal: Port cannot receive inputs.");
    Ok(())
}

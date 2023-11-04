mod controller;
mod serializer;
mod settings;
mod view;
use crate::controller::unix_port::{UnixSocketPort, UnixSocketSettings};
use crate::controller::InputPort;
use crate::settings::{configure, ViewMode};
use crate::view::{HttpView, MockView, View};
use colored::Colorize;
use log::Level;

static SETTINGS_ADDRESS: &str = "./resources/settings.json";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    let configs = configure(SETTINGS_ADDRESS)?;

    log::debug!("Loaded settings from: {}", SETTINGS_ADDRESS.cyan());

    log::info!(
        "Dispatcher path chosen was: {} ; Chosen view method was: {} ; Http requests will be forwarded to {}",
        configs.dispatcher_directory.red(),
        configs.view_mode.to_string().green(),
        configs.http_destination.yellow()
    );

    let view: Box<dyn View> = match configs.view_mode {
        ViewMode::Http => Box::new(HttpView::new(configs.http_destination.as_str())),
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

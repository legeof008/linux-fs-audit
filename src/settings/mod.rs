use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
static SOCKET_ADDRESS: &str = "/var/run/dispatcher";
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum ViewMode {
    Http,
    Mock,
}
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct StartupSettings {
    #[serde(default = "default_dispatcher_directory")]
    pub(super) dispatcher_directory: String,
    #[serde(default = "default_view_mode")]
    pub(super) view_mode: ViewMode,
}
fn default_dispatcher_directory() -> String {
    return String::from(SOCKET_ADDRESS);
}
fn default_view_mode() -> ViewMode {
    return ViewMode::Mock;
}
pub(crate) fn configure(config_file_path: &str) -> Result<StartupSettings, serde_json::Error> {
    let file = File::open(config_file_path).unwrap();
    serde_json::from_reader(BufReader::new(file))
}

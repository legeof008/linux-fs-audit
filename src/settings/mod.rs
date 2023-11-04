use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use serde::de::Error;

static SOCKET_ADDRESS: &str = "/var/run/dispatcher";
static HTTP_VIEW_DESTINATION: &str = "localhost:8080";

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum ViewMode {
    Http,
    Mock,
}
impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct StartupSettings {
    #[serde(default = "default_dispatcher_directory")]
    pub(super) dispatcher_directory: String,
    #[serde(default = "default_view_mode")]
    pub(super) view_mode: ViewMode,
    #[serde(default = "default_http_dest")]
    pub(super) http_destination: String,
}
fn default_dispatcher_directory() -> String {
    return String::from(SOCKET_ADDRESS);
}
fn default_view_mode() -> ViewMode {
    return ViewMode::Mock;
}
fn default_http_dest() -> String {
    return String::from(HTTP_VIEW_DESTINATION);
}
pub(crate) fn configure(config_file_path: &str) -> Result<StartupSettings, serde_json::Error> {
    let file = match File::open(config_file_path) {
        Ok(f) => {f}
        Err(_) => {return Err(serde_json::Error::custom("Error: settings file not present."))}
    };
    serde_json::from_reader(BufReader::new(file))
}

#[cfg(test)]
mod test {
    use crate::settings::{configure, ViewMode};

    #[test]
    fn if_file_present_should_have_correct_settings_set () {
        let read_configs = configure("test_resources/all_present.json").unwrap();
        assert_eq!(read_configs.http_destination,"localhost:9000");
        assert_eq!(read_configs.view_mode,ViewMode::Mock);
        assert_eq!(read_configs.dispatcher_directory,"/var/run/disp");
    }
    #[test]
    fn if_file_not_present_should_be_error () {
        let read_configs = configure("test_resources/no_such_file.json");
        assert!(read_configs.is_err());
    }
    #[test]
    fn if_file_present_should_have_dispatcher_present_others_on_default () {
        let read_configs = configure("test_resources/dispatcher_present.json").unwrap();
        assert_eq!(read_configs.http_destination,"localhost:8080");
        assert_eq!(read_configs.view_mode,ViewMode::Mock);
        assert_eq!(read_configs.dispatcher_directory,"/var/run/disp");
    }
    #[test]
    fn if_file_present_should_have_http_present_others_on_default () {
        let read_configs = configure("test_resources/http_present.json").unwrap();
        assert_eq!(read_configs.http_destination,"localhost:9000");
        assert_eq!(read_configs.view_mode,ViewMode::Mock);
        assert_eq!(read_configs.dispatcher_directory,"/var/run/dispatcher");
    }
    #[test]
    fn if_file_present_should_have_view_present_others_on_default () {
        let read_configs = configure("test_resources/view_present.json").unwrap();
        assert_eq!(read_configs.http_destination,"localhost:8080");
        assert_eq!(read_configs.view_mode,ViewMode::Http);
        assert_eq!(read_configs.dispatcher_directory,"/var/run/dispatcher");
    }
}

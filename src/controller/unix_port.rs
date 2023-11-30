use crate::controller::InputPort;
use crate::encode;
use crate::serializer::{FileOperatedOn, Operation};
use crate::view::View;
use async_trait::async_trait;
use colored::Colorize;
use tokio::io;
use tokio::io::Interest;
use tokio::net::UnixStream;

const STREAM_MAX_SIZE_IN_BYTES: usize = 512;
const INITIAL_BUFFER_VALUE: u8 = 0;

pub(crate) struct UnixSocketPort {
    socket_path: String,
    view: Box<dyn View>,
}

pub(crate) struct UnixSocketSettings {
    pub socket_path: String,
}

#[async_trait]
impl InputPort for UnixSocketPort {
    async fn receive(&self) -> io::Result<()> {
        let data_stream_from_unix_socket = UnixStream::connect(&self.socket_path).await?;
        loop {
            let stream_status = data_stream_from_unix_socket
                .ready(Interest::READABLE)
                .await?;

            let mut read_data = vec![INITIAL_BUFFER_VALUE; STREAM_MAX_SIZE_IN_BYTES];
            let mut previous_timestamp = String::new();
            if stream_status.is_readable() {
                log::info!("Unix stream is readable.");
                match data_stream_from_unix_socket.try_read(&mut read_data) {
                    Ok(_) => {
                        let encoded_values = encode!(read_data);
                        log::debug!("Received message: {}", encoded_values);
                        let operation = Operation::new(encoded_values.clone());
                        let files_changed =
                            FileOperatedOn::new(encoded_values, previous_timestamp.clone());
                        if files_changed.is_some() {
                            log::debug!(
                                "{}: {:?}",
                                "File operated on".green(),
                                operation.iter().clone()
                            );
                        }

                        if operation.is_some() {
                            log::debug!(
                                "{}: {:?}",
                                "Operation observed".green(),
                                operation.iter().clone()
                            );
                            previous_timestamp =
                                operation.iter().clone().nth(0).unwrap().timestamp.clone();
                            log::debug!("{} : {}", "Previous timestamp".cyan(), previous_timestamp);
                            self.view
                                .update(operation.unwrap())
                                .await
                                .expect("Error: rendering view is impossible.");
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        log::warn!("{}", "Blocking error while reading from socket".yellow());
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }
}

impl UnixSocketPort {
    pub(crate) fn new(
        init_settings: UnixSocketSettings,
        output_view: Box<dyn View>,
    ) -> Box<UnixSocketPort> {
        return Box::new(UnixSocketPort {
            socket_path: String::from(init_settings.socket_path),
            view: output_view,
        });
    }
    fn ascii_encode_and_join(read_data: Vec<u8>) -> String {
        read_data.iter().map(|x| *x as char).collect()
    }
}

mod unix_port_macros {
    #[macro_export]
    macro_rules! encode {
        ($x:ident) => {
            UnixSocketPort::ascii_encode_and_join($x)
        };
    }
}

#[cfg(test)]
mod test {
    use crate::controller::unix_port::{UnixSocketPort, UnixSocketSettings};
    use crate::view::MockView;

    #[test]
    fn should_construct_with_correct_path() {
        let path = "path".to_string();
        let config = UnixSocketSettings {
            socket_path: path.clone(),
        };
        let port = UnixSocketPort::new(config, Box::new(MockView {}));
        assert_eq!(port.socket_path, path)
    }
}

use crate::controller::InputPort;
use crate::serializer::Operation;
use async_trait::async_trait;
use tokio::io;
use tokio::io::Interest;
use tokio::net::UnixStream;

const STREAM_MAX_SIZE_IN_BYTES: usize = 470;
const INITIAL_BUFFER_VALUE: u8 = 0;

pub(crate) struct UnixSocketPort {
    socket_path: String,
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

            if stream_status.is_readable() {
                match data_stream_from_unix_socket.try_read(&mut read_data) {
                    Ok(_) => {
                        let encoded_values = Self::ascii_encode_and_join(&mut read_data);
                        let operation = Operation::new(encoded_values);
                        println!("{:?}", operation);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
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
    pub(crate) fn new(init_settings: UnixSocketSettings) -> Self {
        return UnixSocketPort {
            socket_path: String::from(init_settings.socket_path),
        };
    }
    fn ascii_encode_and_join(read_data: &mut Vec<u8>) -> String {
        read_data.iter().map(|x| *x as char).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::controller::unix_port::{UnixSocketPort, UnixSocketSettings};

    #[test]
    fn should_construct_with_correct_path() {
        let path = "path".to_string();
        let config = UnixSocketSettings {
            socket_path: path.clone(),
        };
        let port = UnixSocketPort::new(config);
        assert_eq!(port.socket_path, path)
    }
}

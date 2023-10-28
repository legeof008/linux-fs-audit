use crate::port::InputPort;
use async_trait::async_trait;
use snailquote::unescape;
use std::collections::HashMap;
use tokio::io;
use tokio::io::Interest;
use tokio::net::UnixStream;

const STREAM_MAX_SIZE_IN_BYTES: usize = 256;
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
                        Self::present_output(&mut read_data);
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

    fn present_output(read_data: &mut Vec<u8>) {
        let coded_data: String = Self::ascii_encode_and_join(read_data);
        let map_of_audit_information = coded_data
            .split(" ")
            .into_iter()
            .map(|unsplit_pair| Self::split_by_key_and_value(unsplit_pair))
            .filter(|tuple_of_strings| !tuple_of_strings.1.is_empty())
            .map(|tuple_of_strings| {
                (
                    tuple_of_strings.0,
                    Self::reduce_equal_signs(tuple_of_strings.1),
                )
            })
            .collect::<HashMap<_, _>>();
        if map_of_audit_information.contains_key("key") {
            println!("{:?}", map_of_audit_information);
            println!(
                "Kind of operation: {}",
                unescape(map_of_audit_information.get("key").unwrap()).unwrap()
            );
        }
    }

    fn split_by_key_and_value(x: &str) -> (&str, &str) {
        x.split_at(x.find('=').or(Option::from(0)).unwrap())
    }

    fn reduce_equal_signs(x: &str) -> &str {
        return &x[x.find('=').or(Option::from(0)).unwrap() + 1..];
    }

    fn ascii_encode_and_join(read_data: &mut Vec<u8>) -> String {
        read_data.iter().map(|x| *x as char).collect()
    }
}

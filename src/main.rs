use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut unix_socket_stream = UnixStream::connect("/var/run/dispatcher")?;
    unix_socket_stream.write_all(b"ayee\n")?;
    let mut data_read_from_socket = String::new();
    unix_socket_stream.read_to_string(&mut data_read_from_socket)?;
    println!("{data_read_from_socket}");
    Ok(())
}

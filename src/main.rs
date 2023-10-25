use std::io;
use tokio::io::Interest;
use tokio::net::UnixStream;
static SOCKET_ADDRESS: &str = "";
#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = UnixStream::connect(SOCKET_ADDRESS).await?;
    loop {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;
        if ready.is_readable() {
            let mut data = vec![0; 256];
            match stream.try_read(&mut data) {
                Ok(n) => {
                    let read_message: String = data.iter().map(|x| *x as char).collect();
                    println!("read {} bytes", n);
                    println!("Message = {}", read_message);
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

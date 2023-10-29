use async_trait::async_trait;
use tokio::io;

pub mod unix_port;

#[async_trait]
pub(crate) trait InputPort {
    async fn receive(&self) -> io::Result<()>;
}

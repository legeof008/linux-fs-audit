use async_trait::async_trait;
use tokio::io;

pub mod unix_port;

#[async_trait]
pub(crate) trait InputPort: Send + Sync {
    async fn receive(&self) -> io::Result<()>;
}

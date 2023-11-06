mod http_view;
mod mock_view;
mod sqlite_view;

use crate::serializer::Operation;
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub(crate) trait View: Send + Sync {
    async fn update(&self, operation: Operation) -> Result<(), ()>;
}

pub(crate) struct HttpView {
    destination_url: String,
    client: Client,
}
pub(crate) struct SqliteView {
    db_path: String,
}
pub(crate) struct MockView {}

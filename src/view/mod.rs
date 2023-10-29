mod http_view;
mod mock_view;

use async_trait::async_trait;
use reqwest::Client;
use crate::serializer::Operation;

#[async_trait]
pub(crate) trait View: Send + Sync {
    async fn update(&self, operation: Operation) -> Result<(), ()>;
}

pub(crate) struct HttpView {
    destination_url: String,
    client: Client,
}

pub(crate) struct MockView {}

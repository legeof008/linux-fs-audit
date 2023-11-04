use crate::serializer::Operation;
use crate::view::{HttpView, View};
use async_trait::async_trait;

impl HttpView {
    pub(crate) fn new(destination_url: &str) -> Self {
        return Self {
            destination_url: destination_url.to_string(),
            client: reqwest::Client::new(),
        };
    }
}

#[async_trait]
impl View for HttpView {
    async fn update(&self, operation: Operation) -> Result<(), ()> {
        let jsonized_operation = serde_json::to_string(&operation).unwrap();
        let _resp = self
            .client
            .post(self.destination_url.to_string())
            .body(jsonized_operation)
            .send()
            .await;
        return Ok(());
    }
}

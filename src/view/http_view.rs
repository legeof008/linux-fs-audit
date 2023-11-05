use crate::serializer::Operation;
use crate::view::{HttpView, View};
use async_trait::async_trait;
use colored::Colorize;
use reqwest::header::{CONNECTION, CONTENT_TYPE};

const CONNECTION_HEADER_VALUE: &str = "keep-alive";
const CONTENT_HEADER_VALUE: &str = "application/json";

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
            .header(CONNECTION, CONNECTION_HEADER_VALUE)
            .header(CONTENT_TYPE, CONTENT_HEADER_VALUE)
            .send()
            .await;
        if _resp.is_ok() {
            log::info!("{}", "Operation has been reported".green())
        } else {
            log::error!("{}", "Operation has not been reported".red())
        }
        return Ok(());
    }
}

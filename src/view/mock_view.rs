use crate::serializer::Operation;
use crate::view::{MockView, View};
use async_trait::async_trait;

#[async_trait]
impl View for MockView {
    async fn update(&self, operation: Operation) -> Result<(), ()> {
        log::debug!(
            "Operation parsed to a json: {}",
            serde_json::to_string(&operation).unwrap()
        );
        return Ok(());
    }
}

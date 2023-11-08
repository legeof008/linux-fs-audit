use crate::serializer::Operation;
use crate::view::{HttpView, View};
use async_trait::async_trait;
use colored::Colorize;

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
        log::debug!(
            "Sending {} to endpoint {}",
            jsonized_operation.blue(),
            self.destination_url.to_string().green()
        );
        let _resp = self
            .client
            .post(self.destination_url.to_string())
            .body(jsonized_operation)
            .send()
            .await;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use crate::serializer::Operation;
    use crate::view::{HttpView, View};
    const COMPLIANT_LOG_LINE: &str = "type=SYSCALL msg=audit(1698576562.955:570): arch=c000003e syscall=257 success=yes exit=3 a0=ffffff9c a1=55a917750550 a2=90800 a3=0 items=1 ppid=20120 pid=20680 auid=1000 uid=1000 gid=1000 euid=1000 suid=1000 fsuid=1000 egid=1000 sgid=1000 fsgid=1000 tty=pts2 ses=14 comm=\"ls\" exe=\"/usr/bin/ls\" subj=unconfined key=\"READ\"ARCH=x86_64 AUID=\"maciek\" UID=\"maciek\" GID=\"maciek\" EUID=\"maciek\" SUID=\"maciek\" FSUID=\"maciek\" EGID=\"maciek\" SGID=\"maciek\"";

    #[tokio::test]
    async fn when_sending_operation_msg_server_should_receive_request() {
        // Mock server setup
        let mut destination_server = mockito::Server::new();
        let mut url = destination_server.url();
        url.push_str("/operations");

        // Creating mock
        let mock = destination_server
            .mock("POST", "/operations")
            .with_status(201)
            .create();

        // Reporting an operation
        let http_view = HttpView::new(url.as_str());
        let response = http_view
            .update(Operation::new(COMPLIANT_LOG_LINE.to_string()).unwrap())
            .await;
        assert!(response.is_ok());
        mock.assert();
    }
}

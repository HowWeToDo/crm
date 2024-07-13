use super::{to_ts, Sender};
use crate::{
    pb::{send_request::Msg, InAppMessage, SendRequest, SendResponse},
    NotificationService,
};
use tonic::Status;
use tracing::warn;

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.device_id.clone();
        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        use uuid::Uuid;
        Self {
            device_id: Uuid::new_v4().to_string(),
            title: "Hello".to_string(),
            body: "Hello World".to_string(),
        }
    }
}

impl From<InAppMessage> for Msg {
    fn from(value: InAppMessage) -> Self {
        Msg::InApp(value)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(value: InAppMessage) -> Self {
        let msg: Msg = value.into();
        Self { msg: Some(msg) }
    }
}

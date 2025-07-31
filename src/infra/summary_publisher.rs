use async_trait::async_trait;
use traq::apis::{configuration::Configuration, message_api::post_message};
use traq::models::PostMessageRequest;
use uuid::Uuid;

use crate::domain::infra::SummaryPublisher;

pub struct TraqSummaryPublisher {
    channel_id: Uuid,
    configuration: Configuration,
}

impl TraqSummaryPublisher {
    pub fn new(channel_id: Uuid, access_token: String) -> Self {
        let configuration = Configuration {
            bearer_access_token: Some(access_token),
            ..Default::default()
        };
        Self {
            channel_id,
            configuration,
        }
    }
}

#[async_trait]
impl SummaryPublisher for TraqSummaryPublisher {
    async fn publish_summary(&self, summary: String) -> anyhow::Result<()> {
        post_message(
            &self.configuration,
            self.channel_id.to_string().as_str(),
            Some(PostMessageRequest {
                content: summary,
                ..Default::default()
            }),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to post message: {}", e))?;
        Ok(())
    }
}

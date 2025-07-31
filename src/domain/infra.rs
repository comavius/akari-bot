use super::models::{AkariScore, User};
use anyhow::Result;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait AkariScoreRepository {
    async fn try_save_score(&self, score: AkariScore, user_id: uuid::Uuid) -> Result<()>;
    async fn get_scores(&self) -> Result<HashMap<uuid::Uuid, AkariScore>>;
    async fn refresh_all_scores(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>>;
}

#[async_trait::async_trait]
pub trait SummaryPublisher {
    async fn publish_summary(&self, summary: String) -> Result<()>;
}

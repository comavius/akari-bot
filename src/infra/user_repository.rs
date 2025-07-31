use crate::domain::{infra::UserRepository, models::User};
use anyhow::{Context, Result};
use traq::apis::configuration::Configuration;
use traq::apis::user_api;

#[derive(Clone)]
pub struct TraqUserRepository {
    config: Configuration,
}

impl TraqUserRepository {
    pub fn new() -> Self {
        TraqUserRepository {
            config: Configuration::new(),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for TraqUserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = user_api::get_users(&self.config, None, None)
            .await
            .context("Failed to fetch users from TRAQ API")?;

        let users = users
            .into_iter()
            .map(|user| User {
                name: user.name,
                id: user.id,
            })
            .collect();

        Ok(users)
    }
}

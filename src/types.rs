use serde::Deserialize;

use crate::config::MinebotConfig;

pub type MinebotPoiseContext<'a> = poise::Context<'a, MinebotConfig, MinebotError>;

pub type MinebotError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize)]
pub struct MessagePayload {
    pub channel_id: u64,
    pub content: String,
    pub user: String,
}

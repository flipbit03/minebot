use crate::config::MinebotConfig;

pub type MinebotPoiseContext<'a> = poise::Context<'a, MinebotConfig, MinebotError>;

pub type MinebotError = Box<dyn std::error::Error + Send + Sync>;

mod commands;
mod config;
mod types;

use crate::commands::status::server_status;
use crate::config::load_config;
use dotenv::dotenv;
use serenity::all::{ClientBuilder, GuildId};
use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let c = load_config();
    let c_for_framework = c.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![server_status()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(c_for_framework.guild_id),
                )
                .await?;
                Ok(c_for_framework)
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();
    let client = ClientBuilder::new(&c.discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

mod commands;
mod config;
mod event_handler;
mod types;
use std::sync::Arc;

use crate::commands::status::server_status;
use crate::config::load_config;
use axum::routing::post;
use axum::{Extension, Router};
use dotenv::dotenv;
use http::handlers::send_message;
use serenity::all::{ClientBuilder, GuildId};
use serenity::prelude::GatewayIntents;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
mod http;

#[derive(Clone)]
struct BotState {
    http: Arc<serenity::http::Http>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let c = load_config();
    let c_for_framework = c.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![server_status()],
            event_handler: |_, event, framework, _| {
                Box::pin(event_handler::event_handler(event, framework))
            },
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
    let mut client = ClientBuilder::new(&c.discord_token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    let http = client.http.clone();
    let bot_state = Arc::new(Mutex::new(BotState { http }));

    tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });

    let app = Router::new()
        .route("/send_message", post(send_message))
        .layer(Extension(bot_state));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

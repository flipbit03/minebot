use std::sync::Arc;

use axum::Json;
use serenity::{
    all::{ChannelId, CreateEmbed, CreateMessage},
    model::colour,
};
use tokio::sync::Mutex;

use crate::{types::MessagePayload, BotState};

pub async fn send_message(
    axum::Extension(state): axum::Extension<Arc<Mutex<BotState>>>,
    Json(payload): Json<MessagePayload>,
) -> Result<String, (axum::http::StatusCode, String)> {
    let state = state.lock().await;
    let http = &state.http;

    match ChannelId::new(payload.channel_id)
        .send_message(
            &http,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("Message from Minecraft")
                    .fields(vec![
                        ("User", &payload.user, false),
                        ("Message", &payload.content, false),
                    ])
                    .color(colour::Colour::from_rgb(100, 255, 100)),
            ),
        )
        .await
    {
        Ok(_) => Ok("Message sent".to_string()),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

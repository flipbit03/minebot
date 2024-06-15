use std::env;

use async_minecraft_ping::ConnectionConfig;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;

use crate::embeds;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let address = env::var("SERVER_IP").unwrap();
    let port = env::var("SERVER_PORT").unwrap();

    let mut config = ConnectionConfig::build(address);
    config = config.with_port(port.parse().unwrap());

    let connection = config.connect().await;

    if let Err(why) = connection {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(embeds::status::error::generate(why.to_string())),
                ),
            )
            .await?;
        return Ok(());
    };

    let connection = connection.unwrap();

    let info = connection.status().await.unwrap();

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embeds::status::success::generate(info.status)),
            ),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("status").description("Show minecraft server status")
}

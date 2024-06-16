use log::info;
use poise::CreateReply;

use crate::commands::status::embeds::status::error::create_failure_embed;
use crate::commands::status::embeds::status::success::create_success_embed;
use crate::types::{MinebotError, MinebotPoiseContext};

mod embeds;
/// Show the current status of the server
#[poise::command(slash_command)]
pub async fn server_status(ctx: MinebotPoiseContext<'_>) -> Result<(), MinebotError> {
    let mc = &ctx.data().minecraft;

    let mc_connection_builder =
        async_minecraft_ping::ConnectionConfig::build(mc.address.clone()).with_port(mc.port);

    // Connect or handle connection error
    info!("Connecting to Minecraft server");
    let mc_connect = match mc_connection_builder.connect().await {
        Ok(c) => c,
        Err(c_err) => {
            ctx.send(CreateReply::default().embed(create_failure_embed(c_err.to_string())))
                .await
                .unwrap();
            // .map_err(|e| Box::new(e))?;
            return Ok(());
        }
    };

    // Get server status or handle error
    info!("Getting server status");
    let mc_status = match mc_connect.status().await {
        Ok(s) => s.status,
        Err(c_err) => {
            ctx.send(CreateReply::default().embed(create_failure_embed(c_err.to_string())))
                .await
                .map_err(Box::new)?;
            return Ok(());
        }
    };

    // Got status, build success embed
    info!("Building success embed");
    let success_embed = create_success_embed(mc_status, &ctx.data().minecraft);
    let success_reply = CreateReply::default().embed(success_embed);
    ctx.send(success_reply).await.map_err(Box::new)?;
    Ok(())
}

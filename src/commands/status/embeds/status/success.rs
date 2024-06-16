use async_minecraft_ping::StatusResponse;
use serenity::{all::CreateEmbed, model::colour};

use crate::config::MinebotMinecraftServerConfig;

pub fn create_success_embed(
    status: StatusResponse,
    mc: &MinebotMinecraftServerConfig,
) -> CreateEmbed {
    let players = match status.players.sample {
        Some(players) => players
            .iter()
            .map(|player| {
                player.name.clone()
            })
            .collect::<Vec<String>>()
            .join("\n"),
        None => String::from("No players online"),
    };

    CreateEmbed::new()
        .title("Server Status")
        .fields(vec![
            ("Status", "Online", true),
            (
                "Player Count",
                format!("{} / {}", status.players.online, status.players.max).as_str(),
                true,
            ),
            ("Players", players.as_str(), false),
            (
                "Server IP",
                format!("{}:{}", mc.address, mc.port).as_str(),
                true,
            ),
            ("Version", status.version.name.as_str(), true),
        ])
        .color(colour::Colour::from_rgb(0, 255, 100))
}

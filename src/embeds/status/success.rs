use std::env;

use async_minecraft_ping::StatusResponse;
use serenity::{all::CreateEmbed, model::colour};

pub fn generate(status: StatusResponse) -> CreateEmbed {
    let players = match status.players.sample {
        Some(players) => players
            .iter()
            .map(|player| {
                let name = player.name.clone();
                name
            })
            .collect::<Vec<String>>()
            .join("\n"),
        None => String::from("No players online"),
    };

    let embed = CreateEmbed::new()
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
                format!(
                    "{}:{}",
                    env::var("SERVER_IP").unwrap(),
                    env::var("SERVER_PORT").unwrap()
                )
                .as_str(),
                true,
            ),
            ("Version", status.version.name.as_str(), true),
        ])
        .color(colour::Colour::from_rgb(0, 255, 100));

    embed
}

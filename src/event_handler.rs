use poise::serenity_prelude;

use crate::{config::MinebotConfig, types::MinebotError};

pub async fn event_handler(
    event: &serenity_prelude::FullEvent,
    _framework: poise::FrameworkContext<'_, MinebotConfig, MinebotError>,
) -> Result<(), MinebotError> {
    match event {
        serenity::all::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}

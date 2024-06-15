use serenity::{all::CreateEmbed, model::colour};

pub fn generate(err_info: String) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title("Server Status")
        .fields(vec![
            ("Status", "Probably Offline", false),
            ("Information", err_info.as_str(), false),
        ])
        .color(colour::Colour::from_rgb(255, 0, 0));

    embed
}

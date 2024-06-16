use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MinebotMinecraftServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinebotConfig {
    pub discord_token: String,
    pub guild_id: u64,
    pub minecraft: MinebotMinecraftServerConfig,
}

pub fn load_config() -> MinebotConfig {
    let c = Config::builder()
        .add_source(Environment::default().prefix("MINEBOT").separator("__"))
        .build()
        .expect("Cannot build config");

    c.try_deserialize().expect("Cannot deserialize config")
}

use dotenv::dotenv;
use std::env;

#[derive(Default)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
}

impl Config {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const SERVER_ADDRESS: &str = "SERVER_ADDRESS";
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        Ok(Self {
            database_url: env::var(Config::DATABASE_URL).map_err(|_| "DATABASE_URL is not set")?,
            server_address: env::var(Config::SERVER_ADDRESS)
                .map_err(|_| "SERVER_ADDRESS is not set")?,
        })
    }
}

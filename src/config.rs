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

mod database {
    use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

    pub async fn configure_database(database_url: String) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(15)
            .connect(database_url.as_str())
            .await
            .expect("Database not connected")
    }
}

pub use database::configure_database;

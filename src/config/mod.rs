use dotenv::dotenv;
use eyre::{Result, WrapErr};

use serde::Deserialize;
use sqlx::postgres::PgPool;

use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        
        color_eyre::install()?;
        
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
            )
            .build()
            .unwrap();

        config.try_deserialize().context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");
        PgPool::connect(&self.database_url).await.context("creating database connection pool")
    }

}
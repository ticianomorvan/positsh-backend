use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_address: String,
    pub postgres: deadpool_postgres::Config,
}

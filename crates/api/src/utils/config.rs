use confik::Configuration;
use serde::Deserialize;

#[derive(Debug, Default, Configuration)]
pub struct AppConfig {
    pub web_server_port: u16,
    pub m2m_auth_registry_base_url: String,
    pub redis_url: String,
    pub num_workers: usize,
    pub log_level: String,
    #[confik(from = DbConfig)]
    pub pg: deadpool_postgres::Config,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct DbConfig(deadpool_postgres::Config);

impl From<DbConfig> for deadpool_postgres::Config {
    fn from(value: DbConfig) -> Self {
        value.0
    }
}

impl confik::Configuration for DbConfig {
    type Builder = Option<Self>;
}
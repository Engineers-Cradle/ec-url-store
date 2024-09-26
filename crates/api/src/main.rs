mod repository;
mod utils;
mod dto;
mod libs;

use confik::{Configuration as _, EnvSource};
use dotenvy::dotenv;

use utils::config::AppConfig;

pub fn env_config() -> AppConfig {
    dotenv().ok();

    let config = AppConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    config
}

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        let _ = libs::http::start_web_server();

        println!("Web Server started");
    })
    .await
    .unwrap();
}
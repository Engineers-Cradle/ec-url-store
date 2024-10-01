use dotenv::dotenv;
use std::env;

pub struct Env {
    pub grpc_host_address: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            grpc_host_address: env::var("GRPC_HOST_ADDRESS")
                .expect("GRPC_HOST_ADDRESS must be set"),
        }
    }
}

pub fn get_env() -> Env {
    Env::new()
}
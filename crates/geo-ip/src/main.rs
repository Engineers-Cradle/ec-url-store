mod libs;
mod utils;

#[tokio::main]
async fn main() {
    let _ = libs::server::start_grpc_server().await;
}
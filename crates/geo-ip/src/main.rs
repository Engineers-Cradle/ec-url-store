mod libs;
mod gen;
mod utils;
mod gen_dep {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./gen/proto_descriptor.bin");
}

#[tokio::main]
async fn main() {
    let _ = libs::server::start_grpc_server().await;
}
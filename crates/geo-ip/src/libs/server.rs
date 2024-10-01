use tonic::transport::Server;

use crate::gen::grpc_geoip;
use crate::libs::grpc::GrpcService;

use crate::gen_dep;

pub async fn start_grpc_server() {
    let service_ref = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(gen_dep::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let env = crate::utils::config::get_env();

    let addr: std::net::SocketAddr = env.grpc_host_address.parse().unwrap();
    let service: GrpcService = GrpcService::default();

    println!("GRPC Server started at {}", addr);

    let _ = Server::builder()
        .add_service(service_ref)
        .add_service(grpc_geoip::geo_ip_server::GeoIpServer::new(service))
        .serve(addr)
        .await;
}
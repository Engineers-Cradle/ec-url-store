use std::time::Duration;
use tonic::transport::Server;
use tonic_health::server::HealthReporter;

use grpc_proto::{gen::grpc_geoip, gen_dep};
use crate::libs::grpc::GrpcService;

async fn twiddle_service_status(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;

        if iter % 2 == 0 {
            reporter.set_serving::<grpc_geoip::geo_ip_server::GeoIpServer<GrpcService>>().await;
        } else {
            reporter.set_not_serving::<grpc_geoip::geo_ip_server::GeoIpServer<GrpcService>>().await;
        };
    }
}

pub async fn start_grpc_server() {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<grpc_geoip::geo_ip_server::GeoIpServer<GrpcService>>()
        .await;

    tokio::spawn(twiddle_service_status(health_reporter.clone()));

    let service_ref = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(gen_dep::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let env = crate::utils::config::get_env();

    let addr: std::net::SocketAddr = env.grpc_host_address.parse().unwrap();
    let service: GrpcService = GrpcService::default();

    println!("GRPC Server started at {}", addr);

    let _ = Server::builder()
        .add_service(health_service)
        .add_service(service_ref)
        .add_service(grpc_geoip::geo_ip_server::GeoIpServer::new(service))
        .serve(addr)
        .await;
}
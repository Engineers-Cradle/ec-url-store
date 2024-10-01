use tonic::{self, Status};
use grpc_proto::gen::grpc_geoip::{GeoIpRequest, GeoIpResponse, geo_ip_client::GeoIpClient};

use crate::env_config;

pub async fn read_ip_grpc(
    ip: String,
) -> Result<GeoIpResponse, Status> {
    if ip == "127.0.0.1" || ip == "localhost" || ip == "::1" {
        return Ok(GeoIpResponse {
            city: "Localhost".to_string(),
            country: "Localhost".to_string(),
        });
    }
    
    let client_addr = env_config().grpc_server_address;
    let client = GeoIpClient::connect(client_addr).await;

    let request = tonic::Request::new(GeoIpRequest {
        ip
    });
    
    match client {
        Ok(mut client) => {
            let response = client.get_info(request).await;
            match response {
                Ok(response) => Ok(response.into_inner()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Status::new(
            tonic::Code::Unavailable,
            format!("Failed to connect to server: {}", e),
        )),
    }
}
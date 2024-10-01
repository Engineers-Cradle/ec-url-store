use tonic::{Request, Response, Status};
use std::net::IpAddr;
use maxminddb::geoip2;

use grpc_proto::gen::grpc_geoip;

#[derive(Default)]
pub struct GrpcService {}

#[tonic::async_trait]
impl grpc_geoip::geo_ip_server::GeoIp for GrpcService {
    async fn get_info(
        &self,
        request: Request<grpc_geoip::GeoIpRequest>,
    ) -> Result<Response<grpc_geoip::GeoIpResponse>, Status> {
        let ip: IpAddr = request.get_ref().ip.parse().unwrap();

        let reader: maxminddb::Reader<Vec<u8>> = maxminddb::Reader::open_readfile(
            "db/GeoLite2-City.mmdb",
        )
        .unwrap();


        let city: geoip2::City = reader.lookup(ip).unwrap();
        let country: geoip2::Country = reader.lookup(ip).unwrap();

        let response: grpc_geoip::GeoIpResponse = grpc_geoip::GeoIpResponse {
            city: city.city.unwrap().names.unwrap().get("en").unwrap().to_string(),
            country: country.country.unwrap().names.unwrap().get("en").unwrap().to_string(),
        };

        Ok(Response::new(response))
    }
}
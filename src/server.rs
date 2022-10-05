use std::net::SocketAddr;
use a2s::A2SClient;
use a2s::info::Info;

pub fn make_client() -> A2SClient {
    A2SClient::new().expect("Could not make client")
}

pub fn get_server_info(client: &A2SClient, address: &str) -> Info {
    let addr: SocketAddr = address.parse().expect("Could not parse address");
    client.info(addr).expect("Info get failed")
}
use std::net::SocketAddr;
use a2s::A2SClient;
use a2s::info::Info;
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: A2SClient = A2SClient::new().expect("Could not make client");
}

pub fn get_server_info(address: &str) -> Info {
    let addr: SocketAddr = address.parse().expect("Could not parse address");
    CLIENT.info(addr).expect("Info get failed")
}
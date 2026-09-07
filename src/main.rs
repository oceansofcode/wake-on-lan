use std::str::FromStr;
use std::net::Ipv4Addr;

use tiny_http::{Method, Response, Server, StatusCode};
use serde_json::{Value};
use wol::{MacAddress, send_magic_packet};

const PORT: &str = "5000";

fn main() {
    let server = Server::http("0.0.0.0:".to_string() + PORT).unwrap();

    for mut request in server.incoming_requests() {
        if request.url() == "/wake" && request.method().eq(&Method::Post) {
            let json: Value = serde_json::from_reader(request.as_reader()).unwrap();

            let mac_value = json.get("mac");

            if mac_value.is_some() {
                // Unfortunately JSON Deserialize keeps quotes so substring it

                let value = mac_value.unwrap().to_string();
                let parsed_mac = &value[1..value.len() - 1];
                
                println!("small mac {value} {parsed_mac}");
                let mac_address = MacAddress::from_str(parsed_mac);
                println!("big mac {mac_address:?}");

                send_magic_packet(mac_address.unwrap(), None, (Ipv4Addr::BROADCAST, 9).into()).unwrap();

                request.respond(Response::empty(StatusCode::from(200))).expect("Error!");
            } else {
                request.respond(Response::empty(StatusCode::from(400))).expect("Error!");
            }
        }
    }
}
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;
use std::net::Ipv4Addr;

use crate::Input;

#[derive(Clone, Deserialize, PartialEq)]
pub struct Server {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    name: String,
    station: Ipv4Addr,
    hostname: String,
    pub load: u8,
    status: String,
    locations: Vec<Value>,
    technologies: Vec<Value>,
    groups: Vec<HashMap<String, Value>>,
}

impl Server {
    pub fn country(&self) -> &str {
        self.locations[0]["country"]["name"].as_str().unwrap()
    }
    fn country_code(&self) -> &str {
        self.locations[0]["country"]["code"].as_str().unwrap()
    }
    pub fn city(&self) -> &str {
        self.locations[0]["country"]["city"]["name"]
            .as_str()
            .unwrap()
    }
    fn public_key(&self) -> &str {
        self.technologies[5]["metadata"][0]["value"]
            .as_str()
            .unwrap()
    }
    fn is_p2p(&self) -> bool {
        self.groups[1].get("id").unwrap() == 15
    }
    fn is_wireguard(&self) -> bool {
        self.technologies.get(5).is_some()
    }
    pub fn identifier(&self) -> String {
        self.hostname
            .split('.')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string()
    }
}

pub fn filter_servers(input: &Input, servers: &[Server]) -> Vec<Server> {
    let mut filtered_servers: Vec<Server> = servers
        .iter()
        .filter_map(|x| {
            if x.is_wireguard()
                && x.status == "online"
                && (input.country.is_empty() || x.country() == input.country)
                && (input.country_code.is_empty() || x.country_code() == input.country_code)
                && (input.city.is_empty() || x.city() == input.city)
                && x.is_p2p() == input.p2p
            {
                Some(x.clone())
            } else {
                None
            }
        })
        .collect();

    filtered_servers.sort_unstable_by(|a, b| a.load.cmp(&b.load));

    filtered_servers
}

pub fn generate_config(input: &Input, server: &Server) -> String {
    format!(
        "# Configuration for {0} ({1}) - {2}, {3}
[Interface]
Address = 10.5.0.2/32
PrivateKey = {5}
DNS = {6}
MTU = {7}

[Peer]
PublicKey = {4}
AllowedIPs = 0.0.0.0/0
Endpoint = {0}:51820",
        server.hostname,
        server.station,
        server.city(),
        server.country(),
        server.public_key(),
        input.private_key,
        input.dns,
        input.mtu,
    )
}

pub fn make_qrcode(config: &String) -> Vec<u8> {
    let code = QrCode::new(config).unwrap();
    let image = code.render::<image::Luma<u8>>().build();

    let mut bytes: Vec<u8> = Vec::new();

    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();

    bytes
}

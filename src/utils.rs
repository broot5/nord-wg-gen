use qrcode::QrCode;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;
use std::net::Ipv4Addr;

use crate::app::{ServerFilterParam, UserConfig};

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
    pub fn country_code(&self) -> &str {
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
        self.hostname.split('.').next().unwrap().to_string()
    }
}

pub fn filter_servers(server_filter_params: &ServerFilterParam, servers: &[Server]) -> Vec<Server> {
    let mut filtered_servers: Vec<Server> = servers
        .iter()
        .filter_map(|x| {
            if x.is_wireguard()
                && x.status == "online"
                && (server_filter_params.country.is_empty()
                    || x.country().to_lowercase() == server_filter_params.country.to_lowercase())
                && (server_filter_params.country_code.is_empty()
                    || x.country_code().to_lowercase()
                        == server_filter_params.country_code.to_lowercase())
                && (server_filter_params.city.is_empty()
                    || x.city().to_lowercase() == server_filter_params.city.to_lowercase())
                && x.is_p2p() == server_filter_params.p2p
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

pub fn generate_config(input: &UserConfig, server: &Server) -> String {
    format!(
        "# Configuration for {0} ({1}) - {2}, {3}
[Interface]
Address = 10.5.0.2/32
PrivateKey = {5}
DNS = {6}
MTU = {7}

[Peer]
PublicKey = {4}
AllowedIPs = 0.0.0.0/0, ::/0
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

pub fn get_flag_emoji(country_code: &str) -> Option<String> {
    let base: u32 = 127397;
    let flag: String = country_code
        .to_uppercase()
        .chars()
        .filter_map(|char| std::char::from_u32(base + char as u32))
        .collect();

    if flag.is_empty() {
        None
    } else {
        Some(flag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flag_emoji() {
        assert_eq!(get_flag_emoji("KR"), Some(String::from("🇰🇷")));
        assert_eq!(get_flag_emoji("US"), Some(String::from("🇺🇸")));
        assert_eq!(get_flag_emoji("JP"), Some(String::from("🇯🇵")));
        assert_eq!(get_flag_emoji("DE"), Some(String::from("🇩🇪")));
        assert_eq!(get_flag_emoji("AU"), Some(String::from("🇦🇺")));
    }
}

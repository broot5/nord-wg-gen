use qrcode::QrCode;
use std::io::Cursor;

use crate::app::{ServerFilterParam, UserConfig};
use crate::mapper::Server;

pub fn filter_servers(server_filter_params: &ServerFilterParam, servers: &[Server]) -> Vec<Server> {
    let query_lowercase = server_filter_params.query.to_lowercase();
    let mut filtered_servers: Vec<Server> = servers
        .iter()
        .filter(|server| {
            server.status
                && (query_lowercase.is_empty()
                    || [
                        server.identifier.to_lowercase(),
                        server.country.to_lowercase(),
                        server.city.to_lowercase(),
                    ]
                    .iter()
                    .any(|field| field.contains(&query_lowercase)))
                && server.p2p == server_filter_params.p2p
        })
        .cloned()
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
        server.city,
        server.country,
        server.public_key,
        input.private_key,
        input.dns,
        input.mtu,
    )
}

pub fn make_qrcode(data: &str) -> Vec<u8> {
    let code = QrCode::new(data).unwrap();
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
        .map(|char| std::char::from_u32(base + char as u32).unwrap())
        .collect();

    Some(flag)
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

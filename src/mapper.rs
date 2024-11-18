use std::net::{IpAddr, Ipv4Addr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerIntermediate {
    id: i64,
    // created_at: String,
    // updated_at: String,
    name: String,
    station: String,
    // ipv6_station: String,
    hostname: String,
    load: i32,
    status: String,
    locations: Vec<Location>,
    // services: Vec<Service>,
    technologies: Vec<Technology>,
    groups: Vec<Group>,
    // specifications: Vec<Specification>,
    // ips: Vec<IpEntry>,
}

#[derive(Deserialize, Debug)]
struct Location {
    // id: i64,
    // created_at: String,
    // updated_at: String,
    // latitude: f64,
    // longitude: f64,
    country: Country,
}

#[derive(Deserialize, Debug)]
struct Country {
    // id: i64,
    name: String,
    code: String,
    city: City,
}

#[derive(Deserialize, Debug)]
struct City {
    // id: i64,
    name: String,
    // latitude: f64,
    // longitude: f64,
    // dns_name: String,
    // hub_score: i32,
}

// #[derive(Deserialize, Debug)]
// struct Service {
//     id: i64,
//     name: String,
//     identifier: String,
//     created_at: String,
//     updated_at: String,
// }

#[derive(Deserialize, Debug)]
struct Technology {
    // id: i64,
    // name: String,
    identifier: String,
    // created_at: String,
    // updated_at: String,
    metadata: Vec<Metadata>,
    // pivot: Pivot,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    name: String,
    value: String,
}

// #[derive(Deserialize, Debug)]
// struct Pivot {
//     technology_id: i64,
//     server_id: i64,
//     status: String,
// }

#[derive(Deserialize, Debug)]
struct Group {
    // id: i64,
    // created_at: String,
    // updated_at: String,
    // title: String,
    identifier: String,
    // r#type: GroupType,
}

// #[derive(Deserialize, Debug)]
// struct GroupType {
//     id: i64,
//     created_at: String,
//     updated_at: String,
//     title: String,
//     identifier: String,
// }

// #[derive(Deserialize, Debug)]
// struct Specification {
//     id: i64,
//     title: String,
//     identifier: String,
//     values: Vec<Value>,
// }

// #[derive(Deserialize, Debug)]
// struct Value {
//     id: i64,
//     value: String,
// }

// #[derive(Deserialize, Debug)]
// struct IpEntry {
//     id: i64,
//     created_at: String,
//     updated_at: String,
//     server_id: i64,
//     ip_id: i64,
//     r#type: String,
//     ip: Ip,
// }

// #[derive(Deserialize, Debug)]
// struct Ip {
//     id: i64,
//     ip: String,
//     version: i32,
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Server {
    pub id: usize,
    pub name: String,
    pub station: IpAddr,
    pub hostname: String,
    pub load: u8,
    pub status: bool,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub public_key: String,
    pub p2p: bool,
    pub identifier: String,
}

impl From<&ServerIntermediate> for Server {
    fn from(intermediate: &ServerIntermediate) -> Self {
        Server {
            id: intermediate.id as usize,
            name: intermediate.name.clone(),
            station: IpAddr::V4(intermediate.station.parse::<Ipv4Addr>().unwrap()),
            hostname: intermediate.hostname.clone(),
            load: intermediate.load as u8,
            status: intermediate.status == "online",
            country: intermediate.locations.first().unwrap().country.name.clone(),
            country_code: intermediate.locations.first().unwrap().country.code.clone(),
            city: intermediate
                .locations
                .first()
                .unwrap()
                .country
                .city
                .name
                .clone(),
            public_key: intermediate
                .technologies
                .iter()
                .find(|tech| tech.identifier == "wireguard_udp")
                .and_then(|tech| tech.metadata.iter().find(|meta| meta.name == "public_key"))
                .map_or_else(|| "".to_string(), |meta| meta.value.clone()),
            p2p: intermediate
                .groups
                .iter()
                .any(|group| group.identifier == "legacy_p2p"),
            identifier: intermediate.hostname.split('.').next().unwrap().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_server() -> Result<(), reqwest::Error> {
        let res = reqwest::blocking::Client::new()
            .get(crate::app::URL)
            .send()
            .unwrap()
            .json::<Vec<ServerIntermediate>>()
            .unwrap();

        println!("{:#?}", Server::from(res.iter().next().unwrap()));
        Ok(())
    }
}

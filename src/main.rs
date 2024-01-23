use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

const URL: &str = "https://api.nordvpn.com/v1/servers/recommendations?&filters\\[servers_technologies\\]\\[identifier\\]=wireguard_udp&limit=99999";
const PRIVATE_KEY: &str = "12345privatekey=";
const DNS: &str = "1.1.1.1";

#[derive(Debug, Deserialize)]
struct Server {
    id: usize,
    name: String,
    station: String,
    hostname: String,
    load: usize,
    status: String,
    locations: Vec<Value>,
    technologies: Vec<Value>,
    groups: Vec<HashMap<String, Value>>,
}

impl Server {
    fn country(&self) -> &str {
        self.locations[0]["country"]["name"].as_str().unwrap()
    }
    fn country_code(&self) -> &str {
        self.locations[0]["country"]["code"].as_str().unwrap()
    }
    fn city(&self) -> &str {
        self.locations[0]["country"]["city"]["name"]
            .as_str()
            .unwrap()
    }
    fn wireguard_public_key(&self) -> &str {
        self.technologies[5]["metadata"][0]["value"]
            .as_str()
            .unwrap()
    }
    fn is_p2p(&self) -> bool {
        self.groups[1]
            .get("id")
            .expect("Couldn't parse server's p2p info")
            == 15
    }
}

fn main() {
    //let content = fs::read_to_string("recommendations.json").expect("Couldn't find or load that file.");
    //let mut servers: Vec<Server> = serde_json::from_str(&content).expect("Couldn't read json file.");

    let mut servers: Vec<Server> = reqwest::blocking::get(URL).unwrap().json().unwrap();

    servers.retain(|server| server.status == "online");

    // custom
    servers.retain(|x| x.country_code() == "US");
    //servers.retain(|x| x.country() == "United States");
    //servers.retain(|x| x.city() == "Los Angeles");
    servers.retain(|x| x.is_p2p() == true);

    servers.sort_by(|a, b| a.load.cmp(&b.load));

    let config = format!(
        "# Configuration for {hostname} ({server_ip}) - {city}, {country}
[Interface]
Address = 10.5.0.2/32
PrivateKey = {private_key}
DNS = {dns}

[Peer]
PublicKey = {wireguard_public_key}
AllowedIPs = 0.0.0.0/0
Endpoint = {hostname}:51820",
        hostname = &servers[0].hostname,
        server_ip = &servers[0].station,
        city = &servers[0].city(),
        country = &servers[0].country(),
        private_key = PRIVATE_KEY,
        dns = DNS,
        wireguard_public_key = &servers[0].wireguard_public_key(),
    );

    println!("{config}");
}

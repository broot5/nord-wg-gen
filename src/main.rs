use base64::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

//const URL: &str = "https://api.nordvpn.com/v1/servers/recommendations?&filters\\[servers_technologies\\]\\[identifier\\]=wireguard_udp&limit=99999";
//const URL: &str = "https://api.nordvpn.com/v1/servers/recommendations?&limit=99999";

#[derive(Debug, Deserialize, Clone)]
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
    fn public_key(&self) -> &str {
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

#[derive(Debug)]
struct Input {
    private_key: String,
    country: String,
    city: String,
    p2p: bool,
    dns: String,
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    let servers = use_future(cx, (), |_| async move {
        reqwest::Client::new()
            //.get(URL)
            .get("https://corsproxy.io/?https://api.nordvpn.com/v1/servers/recommendations?&limit=99999")
            //.fetch_mode_no_cors()
            .send()
            .await?
            .json::<Vec<Server>>()
            .await
    });

    let private_key = use_state(cx, || String::new());
    let country = use_state(cx, || String::new());
    let city = use_state(cx, || String::new());
    let p2p = use_state(cx, || true);
    let dns = use_state(cx, || String::from("1.1.1.1"));
    let textarea = use_state(cx, || String::new());
    let qrcode = use_state(cx, || String::new());

    let input = Input {
        private_key: private_key.to_string(),
        country: country.to_string(),
        city: city.to_string(),
        p2p: **p2p,
        dns: dns.to_string(),
    };

    let img_src = format!("data:image/png;base64,{}", qrcode);

    render!(
        div { "nord-wg-gen" }
        div {
            label { r#for: "private_key", "Private Key" }
            input {
                id: "private_key",
                oninput: move |e| {
                    private_key.set(e.value.clone());
                },
                value: "{private_key}"
            }
        }
        div {
            label { r#for: "country", "Country" }
            input {
                id: "country",
                oninput: move |e| {
                    country.set(e.value.clone());
                },
                value: "{country}"
            }
        }
        div {
            label { r#for: "city", "City" }
            input {
                id: "city",
                oninput: move |e| {
                    city.set(e.value.clone());
                },
                value: "{city}"
            }
        }
        div {
            label { r#for: "p2p", "P2P" }
            input {
                id: "p2p",
                r#type: "checkbox",
                oninput: move |e| {
                    p2p.set(e.value.trim().parse().unwrap());
                },
                checked: "{p2p}"
            }
        }
        div {
            label { r#for: "dns", "DNS" }
            input {
                id: "dns",
                oninput: move |e| {
                    dns.set(e.value.clone());
                },
                value: "{dns}"
            }
        }
        div {
            button { onclick: move |_| {
                    let config = generate_config(&input, &servers.value().unwrap().as_ref().unwrap());
                    textarea.set(config.clone());
                    qrcode.set(make_qrcode(&config));
                },
                "Generate"
            }
        }
        div { textarea { value: "{textarea}" } }
        div { button { "Download" } }
        div { img { src: "{img_src}" } }
    )
}

// filter_servers, generate_config
fn generate_config(input: &Input, servers: &Vec<Server>) -> String {
    let mut servers = servers.clone();

    servers.retain(|server| server.status == "online");

    if input.country != "" {
        servers.retain(|x| x.country() == input.country);
    }

    if input.city != "" {
        servers.retain(|x| x.city() == input.city);
    }

    servers.retain(|x| x.is_p2p() == input.p2p);

    if input.country != "" || input.city != "" {
        servers.sort_by(|a, b| a.load.cmp(&b.load));
    }

    if servers.len() == 0 {
        return String::from("Couldn't find a server that meets the requested conditions.");
    }

    format!(
        "# Configuration for {0} ({1}) - {2}, {3}
[Interface]
Address = 10.5.0.2/32
PrivateKey = {5}
DNS = {6}

[Peer]
PublicKey = {4}
AllowedIPs = 0.0.0.0/0
Endpoint = {0}:51820",
        servers[0].hostname,
        servers[0].station,
        servers[0].city(),
        servers[0].country(),
        servers[0].public_key(),
        input.private_key,
        input.dns,
    )
}

fn make_qrcode(config: &String) -> String {
    let code = QrCode::new(config).unwrap();
    let image = code.render::<image::Luma<u8>>().build();

    let mut bytes: Vec<u8> = Vec::new();

    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();

    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

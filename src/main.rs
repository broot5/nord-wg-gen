use base64::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

//const URL: &str = "https://api.nordvpn.com/v1/servers?&limit=99999";
const URL: &str = "https://corsproxy.io/?https://api.nordvpn.com/v1/servers?&limit=99999";

#[derive(Clone, Deserialize)]
struct Server {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    name: String,
    station: String,
    hostname: String,
    load: u8,
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
        self.groups[1].get("id").unwrap() == 15
    }
    fn is_wireguard(&self) -> bool {
        self.technologies.get(5).is_some()
    }
}

struct Input {
    private_key: String,
    country: String,
    country_code: String,
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
            .get(URL)
            .send()
            .await?
            .json::<Vec<Server>>()
            .await
    });

    let private_key = use_state(cx, String::new);
    let country = use_state(cx, String::new);
    let country_code = use_state(cx, String::new);
    let city = use_state(cx, String::new);
    let p2p = use_state(cx, || true);
    let dns = use_state(cx, || String::from("1.1.1.1"));
    let textarea = use_state(cx, String::new);
    let qrcode = use_state(cx, String::new);
    let file_name = use_state(cx, String::new);

    let input = Input {
        private_key: private_key.to_string(),
        country: country.to_string(),
        country_code: country_code.to_string(),
        city: city.to_string(),
        p2p: **p2p,
        dns: dns.to_string(),
    };

    render!(
        h1 { "nord-wg-gen" }
        div {
            label { r#for: "private_key", "Private Key" }
            input {
                id: "private_key",
                r#type: "password",
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
            label { r#for: "country_code", "Country code" }
            input {
                id: "country_code",
                oninput: move |e| {
                    country_code.set(e.value.clone().to_uppercase());
                },
                value: "{country_code}"
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
                list: "dns_list",
                value: "{dns}"
            }
            datalist { id: "dns_list",
                option { value: "1.1.1.1", "Cloudflare(1.1.1.1)" }
                option { value: "9.9.9.9", "Quad9(9.9.9.9)" }
                option { value: "194.242.2.2", "MullvadDNS(194.242.2.2)" }
            }
        }
        div {
            button { onclick: move |_| {
                    match servers.value() {
                        Some(Ok(r)) => {
                            if let Some(server) = filter_servers(&input, r) {
                                let config = generate_config(&input, &server);
                                textarea.set(config.clone());
                                qrcode.set(make_qrcode(&config));
                                file_name.set(server.hostname);
                            } else {
                                textarea
                                    .set(
                                        String::from(
                                            "Couldn't find server that meets the requested conditions.",
                                        ),
                                    );
                            }
                        }
                        Some(Err(_)) => {}
                        None => {}
                    }
                },
                "Generate"
            }
        }

        div { textarea { value: "{textarea}" } }
        div {
            a {
                href: "data:text/plain;base64,{base64::engine::general_purpose::STANDARD.encode(textarea.get())}",
                download: "{file_name}.conf",
                button { "Download" }
            }
        }
        div { img { alt: "QR code", src: "data:image/png;base64,{qrcode}" } }
    )
}

fn filter_servers(input: &Input, servers: &[Server]) -> Option<Server> {
    let mut servers = servers.to_owned();

    servers.retain(|x| x.is_wireguard());

    servers.retain(|x| x.status == "online");

    if !input.country.is_empty() {
        servers.retain(|x| x.country() == input.country);
    }

    if !input.country_code.is_empty() {
        servers.retain(|x| x.country_code() == input.country_code);
    }

    if !input.city.is_empty() {
        servers.retain(|x| x.city() == input.city);
    }

    servers.retain(|x| x.is_p2p() == input.p2p);

    servers.sort_by(|a, b| a.load.cmp(&b.load));

    match servers.is_empty() {
        true => None,
        false => Some(servers[0].clone()),
    }
}

fn generate_config(input: &Input, server: &Server) -> String {
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
        server.hostname,
        server.station,
        server.city(),
        server.country(),
        server.public_key(),
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

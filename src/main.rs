mod utils;

use base64::prelude::*;
use dioxus::prelude::*;

use utils::*;

const URL: &str = "https://corsproxy.io/?https://api.nordvpn.com/v1/servers?&limit=99999";

struct Input {
    private_key: String,
    country: String,
    country_code: String,
    city: String,
    p2p: bool,
    dns: String,
    mtu: String,
    server_index: usize,
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let servers = use_resource(move || async move {
        reqwest::Client::new()
            .get(URL)
            .send()
            .await?
            .json::<Vec<Server>>()
            .await
    });

    let mut private_key = use_signal(String::new);
    let mut country = use_signal(String::new);
    let mut country_code = use_signal(String::new);
    let mut city = use_signal(String::new);
    let mut p2p = use_signal(|| true);
    let mut dns = use_signal(|| String::from("1.1.1.1"));
    let mut mtu = use_signal(|| String::from("1420"));
    let mut server_index = use_signal(|| 0);

    let mut textarea = use_signal(String::new);
    let mut qrcode_bytes = use_signal(Vec::new);
    let mut server_identifier = use_signal(String::new);

    let input = Input {
        private_key: private_key.to_string(),
        country: country.to_string(),
        country_code: country_code.to_string(),
        city: city.to_string(),
        p2p: *p2p.read(),
        dns: dns.to_string(),
        mtu: mtu.to_string(),
        server_index: *server_index.read(),
    };

    rsx! {
        h1 { a { href: "https://github.com/broot5/nord-wg-gen", "nord-wg-gen" } }
        div {
            label { r#for: "private_key", "Private Key" }
            input {
                id: "private_key",
                r#type: "password",
                oninput: move |e| {
                    private_key.set(e.value());
                },
                value: "{private_key}"
            }
        }
        div {
            label { r#for: "country", "Country" }
            input {
                id: "country",
                oninput: move |e| {
                    country.set(e.value());
                },
                value: "{country}"
            }
        }
        div {
            label { r#for: "country_code", "Country code" }
            input {
                id: "country_code",
                oninput: move |e| {
                    country_code.set(e.value().to_uppercase());
                },
                value: "{country_code}"
            }
        }
        div {
            label { r#for: "city", "City" }
            input {
                id: "city",
                oninput: move |e| {
                    city.set(e.value());
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
                    p2p.set(e.value().trim().parse().unwrap());
                },
                checked: "{p2p}"
            }
        }
        div {
            label { r#for: "dns", "DNS" }
            input {
                id: "dns",
                oninput: move |e| {
                    dns.set(e.value());
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
            label { r#for: "mtu", "MTU" }
            input {
                id: "mtu",
                oninput: move |e| {
                    mtu.set(e.value());
                },
                value: "{mtu}"
            }
        }
        div {
            input {
                r#type: "number",
                oninput: move |e| { server_index.set(e.value().trim().parse().unwrap_or_default()) },
                min: "0",
                step: "1",
                pattern: "[0-9]{10}",
                value: "{server_index}"
            }
            button {
                onclick: move |_| {
                    match &*servers.read_unchecked() {
                        Some(Ok(r)) => {
                            if let Some(server) = filter_servers(&input, r) {
                                let config = generate_config(&input, &server);
                                textarea.set(config.clone());
                                qrcode_bytes.set(make_qrcode(&config));
                                server_identifier.set(server.identifier());
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

        div { textarea { value: "{textarea}", readonly: "true" } }
        div {
            a {
                href: "data:text/plain;base64,{base64::engine::general_purpose::STANDARD.encode(&*textarea.read())}",
                download: "nord-{server_identifier}.conf",
                button { "Download" }
            }
        }
        div {
            img {
                alt: "QR Code",
                src: "data:image/png;base64,{base64::engine::general_purpose::STANDARD.encode(&*qrcode_bytes.read())}"
            }
        }
    }
}

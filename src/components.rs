use base64::prelude::*;
use dioxus::prelude::*;

use crate::utils::*;
use crate::{Input, Output, URL};

#[component]
pub fn FormField(
    id: String,
    label_text: String,
    input_type: String,
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        label { r#for: id.clone(), "{label_text}" }
        input {
            id: id.clone(),
            r#type: input_type,
            oninput: move |event| {
                spawn(async move { oninput.call(event) });
            },
            value
        }
    }
}

#[component]
pub fn InputForm() -> Element {
    let mut input = use_context::<Signal<Input>>();

    rsx! {
        div {
            FormField {
                id: "private_key",
                label_text: "Private Key",
                input_type: "password",
                value: input().private_key,
                oninput: move |event: FormEvent| {
                    input.write().private_key = event.value();
                }
            }
        }
        div {
            FormField {
                id: "country",
                label_text: "Country",
                input_type: "text",
                value: input().country,
                oninput: move |event: FormEvent| {
                    input.write().country = event.value();
                }
            }
            FormField {
                id: "country_code",
                label_text: "Country Code",
                input_type: "text",
                value: input().country_code,
                oninput: move |event: FormEvent| {
                    input.write().country_code = event.value().to_uppercase();
                }
            }
        }
        div {
            FormField {
                id: "city",
                label_text: "City",
                input_type: "text",
                value: input().city,
                oninput: move |event: FormEvent| {
                    input.write().city = event.value();
                }
            }
        }
        div {
            label { r#for: "p2p", "P2P" }
            input {
                id: "p2p",
                r#type: "checkbox",
                oninput: move |event| {
                    input.write().p2p = event.value().trim().parse().unwrap();
                },
                checked: input().p2p
            }
        }
        div {
            label { r#for: "dns", "DNS" }
            input {
                id: "dns",
                r#type: "text",
                value: input().dns,
                oninput: move |event| {
                    input.write().dns = event.value();
                },
                list: "dns_list"
            }
            datalist { id: "dns_list",
                option { value: "1.1.1.1", "Cloudflare(1.1.1.1)" }
                option { value: "9.9.9.9", "Quad9(9.9.9.9)" }
                option { value: "194.242.2.2", "MullvadDNS(194.242.2.2)" }
            }
        }
        div {
            FormField {
                id: "mtu",
                label_text: "MTU",
                input_type: "text",
                value: input().mtu,
                oninput: move |event: FormEvent| {
                    input.write().mtu = event.value();
                }
            }
        }
    }
}

#[component]
pub fn Result() -> Element {
    let output = use_context::<Signal<Output>>();

    rsx! {
        textarea { value: "{output.read().config}", readonly: "true" }
        DownloadButton {
            string: output.read().config.clone(),
            file_name: "nord-{output.read().server_identifier}.conf"
        }
        QRCode { bytes: output.read().qrcode_bytes.clone() }
    }
}

#[component]
pub fn DownloadButton(string: String, file_name: String) -> Element {
    rsx! {
        a {
            href: "data:text/plain;base64,{base64::engine::general_purpose::STANDARD.encode(string)}",
            download: file_name,
            button { "Download" }
        }
    }
}

#[component]
pub fn QRCode(bytes: Vec<u8>) -> Element {
    rsx! {
        img {
            alt: "QR Code",
            src: "data:image/png;base64,{base64::engine::general_purpose::STANDARD.encode(bytes)}"
        }
    }
}

#[component]
pub fn ServerList() -> Element {
    let input = use_context::<Signal<Input>>();

    let servers_resource = use_resource(move || async move {
        reqwest::Client::new()
            .get(URL)
            .send()
            .await?
            .json::<Vec<Server>>()
            .await
    });

    match &*servers_resource.read_unchecked() {
        Some(Ok(servers)) => {
            let mut servers = filter_servers(&input(), servers);
            if servers.len() >= 20 {
                servers = servers[..20].to_vec()
            }
            rsx! {
                for server in servers {
                    ServerInfo { input: input(), server: server.clone() }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { "An error occurred while fetching servers {err}" }
        }
        None => rsx! { "Loading servers" },
    }
}

#[component]
pub fn ServerInfo(input: Input, server: Server) -> Element {
    let mut output = use_context::<Signal<Output>>();
    rsx! {
        div {
            button {
                onclick: move |_| {
                    let config = generate_config(&input, &server);
                    *output
                        .write() = Output {
                        config: config.clone(),
                        qrcode_bytes: make_qrcode(&config),
                        server_identifier: server.identifier(),
                    };
                },
                h5 { "{server.identifier()}" }
                p { "{server.city()}, {server.country()} {server.load}" }
            }
        }
    }
}

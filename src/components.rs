use base64::prelude::*;
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use std::time::Duration;

use crate::utils::*;
use crate::{Output, ServerFilterParam, UserConfig, URL};

#[component]
pub fn InputForm() -> Element {
    let mut user_config = use_context::<Signal<UserConfig>>();
    let mut server_filter_param = use_context::<Signal<ServerFilterParam>>();

    rsx! {
        div {
            FormField {
                id: "country",
                label_text: "Country",
                input_type: "text",
                value: &server_filter_param.read().country,
                oninput: move |event: FormEvent| {
                    server_filter_param.write().country = event.value();
                }
            }
            FormField {
                id: "country_code",
                label_text: "Country Code",
                input_type: "text",
                value: &server_filter_param.read().country_code,
                oninput: move |event: FormEvent| {
                    server_filter_param.write().country_code = event.value();
                }
            }
        }
        div {
            FormField {
                id: "city",
                label_text: "City",
                input_type: "text",
                value: &server_filter_param.read().city,
                oninput: move |event: FormEvent| {
                    server_filter_param.write().city = event.value();
                }
            }
        }
        div {
            FormField {
                id: "p2p",
                label_text: "P2P",
                input_type: "checkbox",
                value: "p2p",
                checked: server_filter_param.read().p2p,
                oninput: move |event: FormEvent| {
                    server_filter_param.write().p2p = event.value().trim().parse().unwrap();
                }
            }
        }
        div {
            FormField {
                id: "private_key",
                label_text: "Private Key",
                input_type: "password",
                value: &user_config.read().private_key,
                oninput: move |event: FormEvent| {
                    user_config.write().private_key = event.value();
                }
            }
        }
        div {
            FormField {
                id: "dns",
                label_text: "DNS",
                input_type: "text",
                value: &*user_config.read().dns,
                oninput: move |event: FormEvent| {
                    user_config.write().dns = event.value();
                }
            }
        }
        div {
            FormField {
                id: "mtu",
                label_text: "MTU",
                input_type: "text",
                value: &user_config.read().mtu,
                oninput: move |event: FormEvent| {
                    user_config.write().mtu = event.value();
                }
            }
        }
    }
}

#[component]
pub fn FormField(
    id: &'static str,
    label_text: &'static str,
    input_type: &'static str,
    value: String,
    checked: Option<bool>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let mut debounce = use_debounce(Duration::from_millis(100), move |event| oninput.call(event));

    rsx! {
        label { r#for: id, "{label_text}" }
        input {
            id,
            r#type: input_type,
            value,
            checked,
            oninput: move |event| { debounce.action(event) }
        }
    }
}

#[component]
pub fn ServerList() -> Element {
    let server_filter_param = use_context::<Signal<ServerFilterParam>>();

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
            let filtered_servers = filter_servers(&server_filter_param.read(), servers);

            if filtered_servers.is_empty() {
                return rsx! {
                    p { "Couldn't find server that meets the requested conditions." }
                };
            }

            let servers_iter = filtered_servers.iter().take(20);
            let servers_rendered = servers_iter.map(|server| {
                rsx! {
                    ServerInfo { server: server.clone() }
                }
            });

            rsx! {
                {servers_rendered}
            }
        }
        Some(Err(err)) => {
            rsx! { "An error occurred while fetching servers {err}" }
        }
        None => rsx! { "Loading servers" },
    }
}

#[component]
pub fn ServerInfo(server: Server) -> Element {
    let user_config = use_context::<Signal<UserConfig>>();
    let mut output = use_context::<Signal<Output>>();

    rsx! {
        div {
            button {
                onclick: move |_| {
                    let config = generate_config(&user_config.read(), &server);
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

#[component]
pub fn Result() -> Element {
    let output = use_context::<Signal<Output>>();

    rsx! {
        textarea { value: "{output.read().config}", readonly: "true" }
        DownloadButton {
            string: "{output.read().config}",
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

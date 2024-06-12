use dioxus::prelude::*;

use crate::app::{Output, ServerFilterParam, UserConfig, URL};
use crate::utils::{filter_servers, generate_config, make_qrcode, Server};

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

            let servers_iter = filtered_servers.iter().take(48);
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
        div { class: "card card-compact bg-neutral m-2",
            div { class: "card-body",
                h2 { class: "card-title",
                    "{server.identifier()}"
                    div {
                        class: match server.load {
                            0..=10 => "badge badge-info",
                            11..=30 => "badge badge-success",
                            31..=50 => "badge badge-warning",
                            51..=u8::MAX => "badge badge-error",
                        },
                        "{server.load}%"
                    }
                }
                p { class: "card-body", "{server.city()}, {server.country()}" }
                div { class: "card-actions justify-end",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let config = generate_config(&user_config.read(), &server);
                            *output
                                .write() = Output {
                                config: config.clone(),
                                qrcode_bytes: make_qrcode(&config),
                                server_identifier: server.identifier(),
                            };
                            eval("server_dialog.showModal();").send("Open dialog".into()).unwrap();
                        },
                        "Select"
                    }
                }
            }
        }
    }
}

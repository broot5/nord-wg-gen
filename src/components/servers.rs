use dioxus::prelude::*;

use crate::app::{Output, ServerFilterParam, UserConfig, URL};
use crate::utils::{filter_servers, generate_config, get_flag_emoji, make_qrcode, Server};

#[component]
pub fn ServerList() -> Element {
    let server_filter_param = use_context::<Signal<ServerFilterParam>>();

    let servers_resource = use_resource(|| async move {
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
                    div { class: "hero min-h-screen",
                        div { class: "hero-content text-center",
                            div { class: "max-w-md",
                                div { class: "text-4xl", "No server found" }
                                p { class: "py-6", "No servers were found that match your criteria." }
                            }
                        }
                    }
                };
            }

            let servers_iter = filtered_servers.iter().take(24);
            let servers_rendered = servers_iter.map(|server| {
                rsx! {
                    ServerInfo { server: server.clone() }
                }
            });

            rsx! {
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
                    {servers_rendered}
                }
            }
        }
        Some(Err(err)) => {
            rsx! { "An error occurred while fetching servers {err}" }
        }
        None => {
            rsx! {
                div { class: "hero min-h-screen",
                    div { class: "hero-content text-center",
                        div { class: "max-w-md",
                            div { class: "text-4xl", "Loading..." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ServerInfo(server: Server) -> Element {
    let user_config = use_context::<Signal<UserConfig>>();
    let mut output = use_context::<Signal<Output>>();

    let server_load = server.load;

    rsx! {
        div { class: "stats bg-base-200 shadow-lg m-2",
            button {
                onclick: move |_| {
                    let config = generate_config(&user_config.read(), &server);
                    *output.write() = Output {
                        config: config.clone(),
                        qrcode_bytes: make_qrcode(&config),
                        server_identifier: server.identifier(),
                    };
                    eval("server_dialog.showModal();").send("Open dialog".into()).unwrap();
                },
                div { class: "stat",
                    div { class: "stat-title flex justify-between",
                        div { "{server.identifier().to_uppercase()}" }
                        div { class: match server_load {
                                0..=10 => "badge badge-info",
                                11..=30 => "badge badge-success",
                                31..=50 => "badge badge-warning",
                                51..=u8::MAX => "badge badge-error",
                            }, "{server.load}%" }
                    }
                    div { class: "text-xl flex place-items-start text-wrap", "{server.city()}" }
                    div { class: "stat-desc flex place-items-start text-4xl",
                        "{get_flag_emoji(server.country_code()).unwrap()}"
                    }
                }
            }
        }
    }
}

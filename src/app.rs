use dioxus::prelude::*;

use crate::components::{input::InputForm, result::Result, servers::ServerList};

pub struct UserConfig {
    pub private_key: String,
    pub dns: String,
    pub mtu: String,
}

pub struct ServerFilterParam {
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub p2p: bool,
}

pub struct Output {
    pub config: String,
    pub qrcode_bytes: Vec<u8>,
    pub server_identifier: String,
}

pub const URL: &str = "https://corsproxy.io/?https://api.nordvpn.com/v1/servers?&limit=99999";

#[component]
pub fn App() -> Element {
    use_context_provider(|| {
        Signal::new(UserConfig {
            private_key: String::new(),
            dns: String::from("1.1.1.1"),
            mtu: String::from("1420"),
        })
    });

    use_context_provider(|| {
        Signal::new(ServerFilterParam {
            country: String::new(),
            country_code: String::new(),
            city: String::new(),
            p2p: true,
        })
    });

    use_context_provider(|| {
        Signal::new(Output {
            config: String::new(),
            qrcode_bytes: Vec::new(),
            server_identifier: String::new(),
        })
    });

    rsx! {
        header {
            nav { class: "navbar",
                a {
                    class: "btn btn-ghost text-xl",
                    href: "https://github.com/broot5/nord-wg-gen",
                    "nord-wg-gen"
                }
            }
        }
        main { class: "sm:container sm:mx-auto",
            div { class: "sm:flex",
                div { class: "sm:flex-none", InputForm {} }
                div { class: "sm:flex-auto", ServerList {} }
            }
            div { Result {} }
        }
    }
}

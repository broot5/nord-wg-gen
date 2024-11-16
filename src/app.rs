use dioxus::prelude::*;

use crate::components::{input::InputForm, navbar::Navbar, result::Result, servers::ServerList};

pub struct UserConfig {
    pub private_key: String,
    pub dns: String,
    pub mtu: String,
}

pub struct ServerFilterParam {
    pub query: String,
    pub p2p: bool,
}

pub struct Output {
    pub config: String,
    pub qrcode_bytes: Vec<u8>,
    pub server_identifier: String,
}

pub const URL: &str = "https://api.nordvpn.com/v1/servers?limit=9999&filters[servers_technologies][identifier]=wireguard_udp";

#[component]
pub fn App() -> Element {
    use_context_provider(|| {
        Signal::new(UserConfig {
            private_key: String::new(),
            dns: String::from("103.86.96.100"),
            mtu: String::from("1420"),
        })
    });

    use_context_provider(|| {
        Signal::new(ServerFilterParam {
            query: String::new(),
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
        header { Navbar {} }
        main { class: "sm:container sm:mx-auto",
            div { class: "sm:flex",
                div { class: "sm:flex-none", InputForm {} }
                div { class: "sm:flex-auto", ServerList {} }
            }
            div { Result {} }
        }
    }
}

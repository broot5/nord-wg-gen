mod components;
mod utils;

use dioxus::prelude::*;

use components::*;

const URL: &str = "https://corsproxy.io/?https://api.nordvpn.com/v1/servers?&limit=99999";

struct UserConfig {
    private_key: String,
    dns: String,
    mtu: String,
}

struct ServerFilterParam {
    country: String,
    country_code: String,
    city: String,
    p2p: bool,
}

struct Output {
    config: String,
    qrcode_bytes: Vec<u8>,
    server_identifier: String,
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
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
        main { class: "container mx-auto",
            div { class: "grid grid-cols-2",
                div { class: "overflow-auto", InputForm {} }
                div { class: "overflow-auto", ServerList {} }
            }
            div { Result {} }
        }
    }
}

mod components;
mod utils;

use dioxus::prelude::*;

use components::*;

const URL: &str = "https://corsproxy.io/?https://api.nordvpn.com/v1/servers?&limit=99999";

#[derive(Clone, PartialEq)]
struct Input {
    private_key: String,
    country: String,
    country_code: String,
    city: String,
    p2p: bool,
    dns: String,
    mtu: String,
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
        Signal::new(Input {
            private_key: String::new(),
            country: String::new(),
            country_code: String::new(),
            city: String::new(),
            p2p: true,
            dns: String::from("1.1.1.1"),
            mtu: String::from("1420"),
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
            nav {
                h1 {
                    a { href: "https://github.com/broot5/nord-wg-gen", "nord-wg-gen" }
                }
            }
        }
        div { InputForm {} }
        div { ServerList {} }
        div { Result {} }
    }
}

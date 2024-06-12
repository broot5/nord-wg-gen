use base64::prelude::*;
use dioxus::prelude::*;

use crate::app::Output;

#[component]
pub fn Result() -> Element {
    let output = use_context::<Signal<Output>>();

    rsx! {
        dialog { id: "server_dialog", class: "modal modal-bottom sm:modal-middle",
            div { class: "modal-box",
                p { class: "py-4",
                    ConfigText { config: "{output.read().config}" }
                    QRCode { bytes: output.read().qrcode_bytes.clone() }
                }
                div { class: "modal-action",
                    form { method: "dialog",
                        DownloadButton {
                            string: "{output.read().config}",
                            file_name: "nord-{output.read().server_identifier}.conf"
                        }
                        button { class: "btn btn-primary m-2", "Close" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ConfigText(config: String) -> Element {
    rsx! {
        div { class: "collapse bg-neutral",
            input { r#type: "checkbox" }
            div { class: "collapse-title text-xl font-medium", "Click here to see raw config file" }
            div { class: "collapse-content",
                p { class: "text-pretty", "{config}" }
            }
        }
    }
}

#[component]
pub fn DownloadButton(string: String, file_name: String) -> Element {
    rsx! {
        a {
            href: "data:text/plain;base64,{base64::engine::general_purpose::STANDARD.encode(string)}",
            download: file_name,
            button { class: "btn btn-accent m-2", "Download {file_name}" }
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

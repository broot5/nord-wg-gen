use base64::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn FormField(
    id: String,
    label_text: String,
    input_type: String,
    value: String,
    oninput: EventHandler<FormEvent>,
    list: Option<String>,
) -> Element {
    rsx! {
        label { r#for: id.clone(), "{label_text}" }
        input {
            id: id.clone(),
            r#type: input_type,
            oninput: move |event| { oninput.call(event) },
            value: value,
            list: list.unwrap_or_default()
        }
    }
}

#[component]
pub fn DownloadButton(textarea: String, server_identifier: String) -> Element {
    rsx! {
        a {
            href: "data:text/plain;base64,{base64::engine::general_purpose::STANDARD.encode(&*textarea)}",
            download: "nord-{server_identifier}.conf",
            button { "Download" }
        }
    }
}

#[component]
pub fn QRCode(bytes: Vec<u8>) -> Element {
    rsx! {
        img {
            alt: "QR Code",
            src: "data:image/png;base64,{base64::engine::general_purpose::STANDARD.encode(&*bytes)}"
        }
    }
}

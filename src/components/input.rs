use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use std::time::Duration;

use crate::app::{ServerFilterParam, UserConfig};

#[component]
pub fn InputForm() -> Element {
    let mut user_config = use_context::<Signal<UserConfig>>();
    let mut server_filter_param = use_context::<Signal<ServerFilterParam>>();

    rsx! {
        div { class: "container",
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
    let mut debounce = use_debounce(Duration::from_millis(200), move |event| oninput.call(event));

    rsx! {
        label {
            class: "form-control label items-start w-full max-w-xs",
            r#for: id,
            "{label_text}"
            input {
                class: if input_type == "checkbox" {
                    "checkbox"
                } else {
                    "input input-bordered w-full max-w-xs"
                },
                id,
                r#type: input_type,
                value,
                checked,
                oninput: move |event| { debounce.action(event) }
            }
        }
    }
}

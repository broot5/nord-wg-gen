use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "flex-1",
                a {
                    class: "btn btn-ghost text-xl",
                    href: "https://github.com/broot5/nord-wg-gen",
                    "nord-wg-gen"
                }
            }
        }
    }
}

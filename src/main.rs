mod app;
mod components;
mod utils;

use dioxus::prelude::*;

use crate::app::App;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

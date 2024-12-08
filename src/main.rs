mod app;
mod components;
mod mapper;
mod utils;

use dioxus::prelude::*;

use crate::app::App;

fn main() {
    launch(App);
}

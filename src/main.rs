mod backend;
mod components;
mod pages;
mod routes;

use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed innit logger");

    dioxus::launch(App);
}

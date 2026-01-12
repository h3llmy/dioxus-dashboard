mod backend;
mod components;
mod pages;
mod routes;
mod hooks;

use crate::{components::toast::ToastProvider, hooks::use_dark_mode, routes::Route};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    use_dark_mode();
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ToastProvider { Router::<Route> {} }
    }
}

#[cfg(feature = "server")]
fn start_server() {
    dotenvy::dotenv().ok();
    
    dioxus::serve(|| async move {
        let router = dioxus::server::router(App);

        info!("Starting in server mode");

        Ok(router)
    });
}

#[cfg(not(feature = "server"))]
fn start_ui() {
    dioxus_sdk_storage::set_dir!();
    info!("Starting in client mode");
    dioxus::launch(App);
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed innit logger");
    
    #[cfg(not(feature = "server"))]
    start_ui();


    #[cfg(feature = "server")]
    start_server();
}

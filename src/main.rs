mod backend;
mod components;
mod pages;
mod routes;
mod hooks;

use crate::{hooks::use_dark_mode, routes::Route};
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
        Router::<Route> {}
    }
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed innit logger");
    
    
    #[cfg(not(feature = "server"))]
    {
        dioxus_sdk_storage::set_dir!();
        info!("Starting in client mode");
        dioxus::launch(App);
    }

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        let router = dioxus::server::router(App);

        info!("Starting in server mode");

        Ok(router)
    });
}

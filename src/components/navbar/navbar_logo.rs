use dioxus::prelude::*;

use crate::routes::Route;

const LOGO_URL: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn NavbarLogo() -> Element {
    rsx! {
        Link {
            class: "flex ms-2 md:me-24",
            to: Route::Dashboard {},

            img {
                alt: "FlowBite Logo",
                class: "h-8 me-3",
                src: LOGO_URL,
            }

            span {
                class: "self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white",
                "Dashboard"
            }
        }
    }
}

use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn AuthFooterButton(text: String, link_text: String, to: Route) -> Element {
    rsx! {
        p { class: "mt-6 text-center text-sm text-gray-600 dark:text-gray-400",
            {text}
            Link { to, class: "text-blue-600 hover:underline font-medium", {link_text} }
        }
    }
}
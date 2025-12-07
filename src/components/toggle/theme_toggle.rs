use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use web_sys::{HtmlElement, window};
use wasm_bindgen::JsCast;

#[component]
pub fn ThemeToggle() -> Element {
    #[cfg(target_arch = "wasm32")]
    let is_dark_mode_local_storage = LocalStorage::get("theme")
        .unwrap_or_else(|_| "light".to_string()) == "dark";
    #[cfg(not(target_arch = "wasm32"))]
    let is_dark_mode_local_storage = true; // fallback for SSR

    let mut dark = use_signal(|| is_dark_mode_local_storage);

    // Sync HTML class and localStorage
    #[cfg(target_arch = "wasm32")]
    {
        use_effect({
            let dark = dark.clone();
            move || {
                let document = window().unwrap().document().unwrap();
                let html: HtmlElement = document.document_element().unwrap().unchecked_into();
                html.set_class_name(if dark() { "dark" } else { "" });
                LocalStorage::set("theme", if dark() { "dark" } else { "light" }).ok();
            }
        });
    }

    rsx! {
        button {
            class: "relative w-20 h-10 flex items-center bg-gray-200 dark:bg-gray-700 rounded-full p-1 transition-colors duration-300 focus:outline-none",
            onclick: move |_| dark.toggle(),

            // The moving circle
            div {
                class: format!(
                    "w-8 h-8 bg-white rounded-full shadow-md transform transition-transform duration-300 {}",
                    if dark() { "translate-x-10" } else { "translate-x-0" }
                )
            }

            // The icon inside the toggle
            div {
                class: "absolute inset-0 flex items-center justify-center text-sm font-semibold text-gray-800 dark:text-gray-200 pointer-events-none",
                if dark() { "☀" } else { "🌙" }
            }
        }
    }
}

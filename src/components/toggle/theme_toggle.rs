use dioxus::prelude::*;
use dioxus_sdk_storage::{LocalStorage, use_synced_storage};
use web_sys::{HtmlElement, window};
use wasm_bindgen::JsCast;

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark: Signal<bool> = use_synced_storage::<LocalStorage, bool>("theme".to_string(), || false);

    use_effect({
        let dark: Signal<bool> = dark.clone();
        move || {
            let document = window().unwrap().document().unwrap();
            let html: HtmlElement = document.document_element().unwrap().unchecked_into();
            html.set_class_name(if dark() { "dark" } else { "" });
        }
    });
    

    rsx! {
        button {
            class: "relative w-20 h-10 flex items-center bg-gray-200 dark:bg-gray-700 rounded-full p-1 transition-colors duration-300 focus:outline-none",
            onclick: move |_| dark.toggle(),

            // The moving circle
            div {
                class: format!(
                    "w-8 h-8 bg-white rounded-full shadow-md transform transition-transform duration-300 {}",
                    if dark() { "translate-x-10" } else { "translate-x-0" },
                ),
            }

            // The icon inside the toggle
            div { class: "absolute inset-0 flex items-center justify-center text-sm font-semibold text-gray-800 dark:text-gray-200 pointer-events-none",
                if dark() {
                    "☀"
                } else {
                    "🌙"
                }
            }
        }
    }
}

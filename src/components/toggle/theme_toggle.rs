use dioxus::prelude::*;

use crate::hooks::use_dark_mode;

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark: Signal<bool> = use_dark_mode();

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

use dioxus::prelude::*;

#[component]
pub fn UserMenuItem(href: String, label: String) -> Element {
    rsx! {
        li {
            Link {
                class: "block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-100
                        dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-white",
                to: href.clone(),
                role: "menuitem",
                {label}
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::navbar::{UserMenuItem, user_menu_items};

#[component]
pub fn UserMenu(user_menu_open: Signal<bool>) -> Element {
    let menu_class = move || {
        let base = "
            absolute right-0 top-full mt-4 w-44
            bg-white dark:bg-gray-700
            z-50 border border-default-medium
            rounded-sm shadow-lg
            transition-all duration-200 ease-out
            transform
        ";

        if user_menu_open() {
            format!("{base} opacity-100 translate-y-0 pointer-events-auto")
        } else {
            format!("{base} opacity-0 -translate-y-2 pointer-events-none")
        }
    };

    rsx! {
        div { class: menu_class(),

            div { class: "px-4 py-3", role: "none",
                p { class: "text-sm text-gray-900 dark:text-white", "Neil Sims" }
                p { class: "text-sm font-medium text-gray-900 truncate dark:text-gray-300",
                    "neil.sims@flowbite.com"
                }
            }

            ul { class: "py-1", role: "none",

                for (label, href) in user_menu_items() {
                    UserMenuItem {
                        href: href.clone(),
                        label: label,
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::sidebar::{SidebarMenu, menu_items};

#[component]
pub fn Sidebar(sidebar_open: ReadSignal<bool>) -> Element {
    let sidebar_class = move || {
        let base: &str = "fixed top-0 left-0 z-40 w-64 h-screen pt-20 bg-white border-r 
                border-gray-200 dark:bg-gray-800 dark:border-gray-700 
                transition-transform duration-300 ease-in-out 
                sm:translate-x-0";

        if sidebar_open() {
            format!("{base} translate-x-0")
        } else {
            format!("{base} -translate-x-full")
        }
    };

    rsx! {
        aside {
            aria_label: "Sidebar",
            class: sidebar_class(),
            id: "logo-sidebar",
            div { class: "h-full px-3 pb-4 overflow-y-auto bg-white dark:bg-gray-800",
                ul { class: "space-y-2 font-medium",
                    for menu in menu_items() {
                        SidebarMenu { menu_item: menu.clone() }
                    }
                }
            }
        }
    }
}
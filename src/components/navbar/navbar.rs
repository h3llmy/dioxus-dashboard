use dioxus::prelude::*;

use crate::components::navbar::{NavbarLogo, NavbarRight, SidebarToggle};

#[component]
pub fn Navbar(sidebar_toggle: WriteSignal<bool>) -> Element {
    let user_menu_open: Signal<bool> = use_signal(|| false);

    rsx! {
        nav { class: "fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700",

            div { class: "px-3 py-3 lg:px-5 lg:pl-3",
                div { class: "flex items-center justify-between",

                    // Left side = sidebar button + logo
                    div { class: "flex items-center justify-start rtl:justify-end",
                        SidebarToggle { sidebar_toggle }
                        NavbarLogo {}
                    }

                    // Right side user menu
                    NavbarRight { user_menu_open }
                }
            }
        }
    }
}

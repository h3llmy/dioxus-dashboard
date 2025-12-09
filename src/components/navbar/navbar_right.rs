use dioxus::prelude::*;

use crate::components::{navbar::{UserAvatar, UserMenu}, toggle::ThemeToggle};

#[component]
pub fn NavbarRight(user_menu_open: Signal<bool>) -> Element {
    rsx! {
        div { class: "flex items-center",
            div { class: "relative flex items-center ms-3 space-x-4 rtl:space-x-reverse",
                ThemeToggle {}
                UserAvatar { user_menu_open }
                UserMenu { user_menu_open }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::navbar::{UserAvatar, UserMenu};

#[component]
pub fn NavbarRight(user_menu_open: Signal<bool>) -> Element {
    rsx! {
        div { class: "flex items-center",
            div { class: "relative flex items-center ms-3",

                UserAvatar { user_menu_open }
                UserMenu { user_menu_open }
            }
        }
    }
}

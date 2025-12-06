use dioxus::prelude::*;
use crate::routes::Route;

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub href: Route,
    pub icon: fn(is_active: bool) -> Element,
}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.href == other.href
    }
}

#[component]
pub fn SidebarMenu(menu_item: MenuItem) -> Element {
    let route: Route = use_route::<Route>();
    let mut path: String = route.to_string();

    // Normalize (Dioxus returns "" for root)
    if path.is_empty() {
        path = "/".to_string();
    }

    // Convert route enum into base route path
    let target: String = menu_item.href.to_string();

    // -------- FIXED ACTIVE CHECK --------
    let is_active: bool = if target == "/" {
        path == "/"
    } else {
        path == target || path.starts_with(&format!("{target}/"))
    };
    // ------------------------------------

    let base = "flex items-center p-2 rounded-lg group transition-colors";
    let active_class = if is_active {
        "bg-gray-200 dark:bg-gray-700 text-blue-600 dark:text-white"
    } else {
        "text-gray-900 dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
    };

    let classes = format!("{base} {active_class}");

    rsx! {
        li {
            Link {
                to: menu_item.href.clone(),
                class: "{classes}",

                {(menu_item.icon)(is_active)}

                span { class: "ms-3", "{menu_item.label}" }
            }
        }
    }
}

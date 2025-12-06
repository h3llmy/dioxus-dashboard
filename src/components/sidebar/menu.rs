use dioxus::prelude::*;

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub href: String,
    pub icon: fn() -> Element,
}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.href == other.href
    }
}

#[component]
pub fn SidebarMenu(menu_item: MenuItem) -> Element {
    rsx! {
        li {
            a {
                class: "flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group",
                href: "{menu_item.href}",
                {(menu_item.icon)()},
                span { class: "ms-3", "{menu_item.label}" }
            }
        }
    }
}
use dioxus::prelude::*;

#[component]
pub fn SidebarToggle(sidebar_toggle: WriteSignal<bool>) -> Element {
    rsx! {
        button {
            aria_controls: "logo-sidebar",
            onclick: move |_| sidebar_toggle.toggle(),
            class: "inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden
                    hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200
                    dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
            r#type: "button",

            span { class: "sr-only", "Open sidebar" }
            icons::logo_icon {}
        }
    }
}

mod icons {
    use super::*;
    pub(crate) fn logo_icon() -> Element {
        rsx! { 
            svg {
                class: "w-6 h-6",
                fill: "currentColor",
                view_box: "0 0 20 20",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    clip_rule: "evenodd",
                    d: "M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z",
                    fill_rule: "evenodd",
                }
            }
        }
    }
}

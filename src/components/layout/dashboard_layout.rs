use crate::components::{navbar::Navbar, sidebar::Sidebar};
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn DashboardLayout() -> Element {
    let sidebar_open: Signal<bool> = use_signal(|| false);

    rsx! {
        Navbar { sidebar_toggle: sidebar_open }

        // This is where the routed content will go
        div { class: "bg-white dark:bg-gray-900 h-screen overflow-auto w-full dark:text-white",
            div { class: "p-4 sm:ml-64",
                div { class: "p-4 mt-14", Outlet::<Route> {} }
            }
        }

        Sidebar { sidebar_open }
    }
}

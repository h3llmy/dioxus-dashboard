use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let nav: dioxus_router::Navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col items-center justify-center h-[100vh] dark:bg-gray-900 text-gray-800 dark:text-gray-200",
            h1 { class: "text-6xl font-bold mb-4", "404" }
            h2 { class: "text-2xl mb-8", "Page Not Found" }
            p { class: "mb-8", "The page you are looking for does not exist or has been moved." }
            button {
                onclick: move |_| nav.go_back(),
                class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",
                "Go to Back"
            }
        }
    }
}
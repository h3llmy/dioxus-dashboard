use dioxus::prelude::*;

/// Kanban page
#[component]
pub fn KanbanDetail(id: String) -> Element {
    rsx! {
        div { "This is the Kanban page detail. ID: {id}" }
    }
}
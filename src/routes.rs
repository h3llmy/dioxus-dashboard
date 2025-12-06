use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(DashboardLayout)]
    #[route("/")]
    Dashboard {},

    #[nest("/kanban")]
        #[route("")]
        Kanban {},
        #[route("/:id")]
        KanbanDetail {
            id: String,
        },
}

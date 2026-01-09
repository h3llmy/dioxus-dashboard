use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},

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
    #[end_nest]
    #[end_layout]
    
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

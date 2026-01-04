use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn PermissionLayout() -> Element {
    // let user = use_context::<User>();

    // match user {
    //     Some(user) if user.has_permission("access_page") => {
    //         rsx! {
    //             children {}
    //         }
    //     }
    //     _ => {
            rsx! {
                div { class: "permission-denied",
                    h1 { "Permission Denied" }
                    p { "You do not have the necessary permissions to access this page." }
                    Outlet::<Route> {}
                }
            }
    //     }
    // }
}
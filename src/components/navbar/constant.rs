use crate::routes::Route;

pub fn user_menu_items() -> Vec<(&'static str, Route)> {
    vec![
        ("Dashboard",  Route::Dashboard { }),
        // ("Settings", "/settings"),
        // ("Earnings", "/earnings"),
        // ("Sign out", "/signout"),
    ]
}
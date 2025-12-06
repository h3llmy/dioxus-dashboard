pub fn user_menu_items() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Dashboard", "/dashboard"),
        ("Settings", "/settings"),
        ("Earnings", "/earnings"),
        ("Sign out", "/signout"),
    ]
}
use dioxus::prelude::*;

#[component]
pub fn Navbar(sidebar_toggle: WriteSignal<bool>) -> Element {
    let mut user_menu_open: Signal<bool> = use_signal(|| false);

    let user_menu_class = move || {
        let base: &str = "
        absolute right-0 top-full mt-4 w-44
        bg-white dark:bg-gray-700
        z-50 border border-default-medium
        rounded-sm shadow-lg
        transition-all duration-200 ease-out
        transform
    ";

        if user_menu_open() {
            format!("{base} opacity-100 translate-y-0 pointer-events-auto")
        } else {
            format!("{base} opacity-0 -translate-y-2 pointer-events-none")
        }
    };

    rsx! {
        nav { class: "fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700",
            div { class: "px-3 py-3 lg:px-5 lg:pl-3",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center justify-start rtl:justify-end",
                        button {
                            aria_controls: "logo-sidebar",
                            onclick: move |_| sidebar_toggle.toggle(),
                            class: "inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                            "data-drawer-target": "logo-sidebar",
                            "data-drawer-toggle": "logo-sidebar",
                            r#type: "button",
                            span { class: "sr-only", "Open sidebar" }
                            icons::logo_icon {}
                        }
                        a {
                            class: "flex ms-2 md:me-24",
                            href: "https://flowbite.com",
                            img {
                                alt: "FlowBite Logo",
                                class: "h-8 me-3",
                                src: "https://flowbite.com/docs/images/logo.svg",
                            }
                            span { class: "self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white",
                                "Flowbite"
                            }
                        }
                    }
                    div { class: "flex items-center",
                        div { class: "relative flex items-center ms-3",
                            div {
                                button {
                                    onclick: move |_| user_menu_open.toggle(),
                                    class: "flex text-sm bg-gray-800 rounded-full cursor-pointer focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600",
                                    "data-dropdown-toggle": "dropdown-user",
                                    r#type: "button",
                                    span { class: "sr-only", "Open user menu" }
                                    img {
                                        alt: "user photo",
                                        class: "w-8 h-8 rounded-full cursor-pointer",
                                        src: "https://flowbite.com/docs/images/people/profile-picture-5.jpg",
                                    }
                                }
                            }

                            div {
                                class: user_menu_class(),

                                div { class: "px-4 py-3", role: "none",
                                    p {
                                        class: "text-sm text-gray-900 dark:text-white",
                                        role: "none",
                                        "Neil Sims"
                                    }
                                    p {
                                        class: "text-sm font-medium text-gray-900 truncate dark:text-gray-300",
                                        role: "none",
                                        "neil.sims@flowbite.com"
                                    }
                                }

                                ul { class: "py-1", role: "none",

                                    li {
                                        a {
                                            class: "block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-white",
                                            href: "#",
                                            role: "menuitem",
                                            "Dashboard"
                                        }
                                    }

                                    li {
                                        a {
                                            class: "block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-white",
                                            href: "#",
                                            role: "menuitem",
                                            "Settings"
                                        }
                                    }

                                    li {
                                        a {
                                            class: "block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-white",
                                            href: "#",
                                            role: "menuitem",
                                            "Earnings"
                                        }
                                    }

                                    li {
                                        a {
                                            class: "block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-white",
                                            href: "#",
                                            role: "menuitem",
                                            "Sign out"
                                        }
                                    }
                                }
                            }
                        }
                    }

                }
            }
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

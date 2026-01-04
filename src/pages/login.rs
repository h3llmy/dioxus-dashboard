use dioxus::prelude::*;

use crate::backend::login;

#[component]
pub fn Login() -> Element {
    let mut show_password: Signal<bool> = use_signal::<bool>(|| false);
    let mut username: Signal<String> = use_signal::<String>(|| "".to_string());
    let mut password: Signal<String> = use_signal::<String>(|| "".to_string());
    let nav = use_navigator();

    rsx! {
        div { class: "
                min-h-screen flex items-center justify-center
                bg-gray-50 dark:bg-gray-900
                px-4
            ",

            div { class: "
                    w-full max-w-md
                    bg-white dark:bg-gray-800
                    rounded-lg shadow-lg
                    p-6 sm:p-8
                ",

                // Header
                div { class: "mb-10 text-center",
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                        "Sign in to your account"
                    }
                }

                // Form
                form {
                    class: "space-y-4",
                    onsubmit: move |evt: FormEvent| async move {
                        evt.prevent_default();

                        match login(username(), password()).await {
                            Ok(_) => {
                                info!("Login successful");
                            }
                            Err(e) => {
                                info!("Login failed: {:?}", e);
                            }
                        }
                    },

                    // Email
                    div {
                        label { class: "block mb-1 text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Email"
                        }
                        input {
                            name: "email",
                            id: "email",
                            r#type: "email",
                            placeholder: "you@example.com",
                            required: true,
                            class: "
                                w-full px-3 py-2
                                rounded-md
                                border border-gray-300 dark:border-gray-600
                                bg-white dark:bg-gray-700
                                text-gray-900 dark:text-white
                                placeholder-gray-400
                                focus:outline-none focus:ring-2 focus:ring-blue-500
                            ",
                        }
                    }

                    // Password
                    div {
                        label { class: "block mb-1 text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Password"
                        }

                        div { class: "relative",
                            input {
                                name: "password",
                                id: "password",
                                r#type: if show_password() { "text" } else { "password" },
                                placeholder: "••••••••",
                                required: true,
                                class: "
                                    w-full px-3 py-2 pr-10
                                    rounded-md
                                    border border-gray-300 dark:border-gray-600
                                    bg-white dark:bg-gray-700
                                    text-gray-900 dark:text-white
                                    focus:outline-none focus:ring-2 focus:ring-blue-500
                                ",
                            }

                            button {
                                r#type: "button",
                                onclick: move |_| show_password.set(!show_password()),
                                class: "
                                    absolute inset-y-0 right-0 px-3
                                    text-sm text-gray-500 dark:text-gray-400
                                    hover:text-gray-700 dark:hover:text-gray-200
                                ",
                                if show_password() {
                                    "Hide"
                                } else {
                                    "Show"
                                }
                            }
                        }
                    }

                    // Remember + Forgot
                    div { class: "flex items-center justify-between",

                        label { class: "flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400",
                            input {
                                r#type: "checkbox",
                                class: "rounded border-gray-300 dark:border-gray-600",
                            }
                            "Remember me"
                        }

                        a {
                            href: "#",
                            class: "text-sm text-blue-600 hover:underline",
                            "Forgot password?"
                        }
                    }

                    // Submit
                    button {
                        r#type: "submit",
                        class: "
                            w-full py-2.5
                            rounded-md
                            bg-blue-600 hover:bg-blue-700
                            text-white font-medium
                            transition-colors
                        ",
                        "Sign In"
                    }
                }

                // Footer
                p { class: "mt-6 text-center text-sm text-gray-600 dark:text-gray-400",
                    "Don’t have an account? "
                    a {
                        href: "/register",
                        class: "text-blue-600 hover:underline font-medium",
                        "Sign up"
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::backend::{LoginRequest, login};

#[component]
pub fn Login() -> Element {
    let mut show_password: Signal<bool> = use_signal(|| false);
    let mut username: Signal<String> = use_signal(|| String::new());
    let mut password: Signal<String> = use_signal(|| String::new());
    let mut loading: Signal<bool> = use_signal(|| false);

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

                        loading.set(true);
                        match login(LoginRequest::new(username(), password())).await {
                            Ok(_) => {
                                info!("Login successful");
                                nav.push(crate::routes::Route::Dashboard {});
                            }
                            Err(e) => {
                                info!("Login failed: {:?}", e);
                            }
                        }
                        loading.set(false);
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
                            value: "{username}",
                            oninput: move |e| username.set(e.value().clone()),
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
                                value: "{password}",
                                oninput: move |e| password.set(e.value().clone()),
                                r#type: if show_password() { "text" } else { "password" },
                                placeholder: "••••••••",
                                required: true,
                                class: "
                                    w-full px-3 py-2 pr-14
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
                        disabled: loading(),
                        class: "
                            w-full py-2.5
                            rounded-md
                            bg-blue-600 hover:bg-blue-700
                            text-white font-medium
                            transition-colors
                            disabled:opacity-50
                            disabled:cursor-not-allowed
                        ",
                        if loading() {
                            "Signing in..."
                        } else {
                            "Sign In"
                        }
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

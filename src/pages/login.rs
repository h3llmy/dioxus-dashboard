use dioxus::prelude::*;

use crate::{backend::{LoginRequest, login}, components::{card::AuthCard, input::{PasswordInput, TextInput}}};

#[component]
pub fn Login() -> Element {
    // let mut username: Signal<String> = use_signal(|| String::new());
    // let mut password: Signal<String> = use_signal(|| String::new());
    let mut username: Signal<String> = use_signal(|| "admin@some.mail".to_string());
    let mut password: Signal<String> = use_signal(|| "password".to_string());
    let mut loading: Signal<bool> = use_signal(|| false);

    let nav = use_navigator();

    rsx! {
        AuthCard { header: Some("Sign in to your account".to_string()),
            // Form
            form {
                class: "space-y-4",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();

                    loading.set(true);
                    match login(LoginRequest {
                            username: username(),
                            password: password(),
                        })
                        .await
                    {
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
                TextInput {
                    value: username(),
                    oninput: move |val| username.set(val),
                    required: true,
                    placeholder: "you@example.com".to_string(),
                    r#type: "email".to_string(),
                    name: "emails".to_string(),
                    label: "Email".to_string(),
                    id: "email".to_string(),
                }

                // Password
                PasswordInput {
                    value: password(),
                    oninput: move |val| password.set(val),
                    required: true,
                    placeholder: "password".to_string(),
                    r#type: "password".to_string(),
                    name: "password".to_string(),
                    label: "Password".to_string(),
                    id: "password".to_string(),
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
                            hover:cursor-pointer
                            focus:outline-none
                            focus:ring-2
                            focus:ring-blue-500
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

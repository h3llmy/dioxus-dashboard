use dioxus::prelude::*;

use crate::{backend::{RegisterRequest, register}, components::{card::AuthCard, input::TextInput, toast::{ToastContext, ToastKind}}, routes::Route};

#[component]
pub fn Register() -> Element {
    let mut username: Signal<String> = use_signal(|| String::new());
    let mut email: Signal<String> = use_signal(|| String::new());
    let mut password: Signal<String> = use_signal(|| String::new());
    let mut loading: Signal<bool> = use_signal(|| false);
    let mut toast_ctx: ToastContext = use_context::<ToastContext>();

    let nav = use_navigator();

    rsx!(
        AuthCard { header: Some("Create a new account".to_string()),
            form {
                class: "space-y-4",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();
                    loading.set(true);
                    match register(RegisterRequest {
                            username: username(),
                            email: email(),
                            password: password(),
                        })
                        .await
                    {
                        Ok(_) => {
                            info!("Registration successful");
                            toast_ctx
                                .push("Registration successful".to_string(), ToastKind::Success);
                            nav.push(Route::Login {});
                        }
                        Err(e) => {
                            toast_ctx
                                .push(
                                    format!(
                                        "Registration failed: {}",
                                        e.message.as_deref().unwrap_or("Something went wrong"),
                                    ),
                                    ToastKind::Error,
                                );
                            info!("Registration failed: {:?}", e);
                        }
                    }
                    loading.set(false);
                },

                // Username
                TextInput {
                    value: username(),
                    oninput: move |val| username.set(val),
                    required: true,
                    placeholder: "yourusername".to_string(),
                    r#type: "text".to_string(),
                    name: "username".to_string(),
                    label: "Username".to_string(),
                    id: "username".to_string(),
                }

                // Email
                TextInput {
                    value: email(),
                    oninput: move |val| email.set(val),
                    required: true,
                    placeholder: "you@example.com".to_string(),
                    r#type: "email".to_string(),
                    name: "email".to_string(),
                    label: "Email".to_string(),
                    id: "email".to_string(),
                }

                // Password
                TextInput {
                    value: password(),
                    oninput: move |val| password.set(val),
                    required: true,
                    placeholder: "password".to_string(),
                    r#type: "password".to_string(),
                    name: "password".to_string(),
                    label: "Password".to_string(),
                    id: "password".to_string(),
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
                Link {
                    to: Route::Login {},
                    class: "text-blue-600 hover:underline font-medium",
                    "Log in here"
                }
            }
        }
    )
}
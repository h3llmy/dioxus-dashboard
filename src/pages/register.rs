use dioxus::prelude::*;

use crate::{backend::{RegisterRequest, register}, components::{button::{AuthButton, AuthFooterButton}, card::AuthCard, input::{PasswordInput, TextInput}, toast::{ToastContext, ToastKind}}, routes::Route};

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
                    let data = evt.data();
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
                    placeholder: "username".to_string(),
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

                // Submit
                AuthButton {
                    disabled: false,
                    label: "Register".to_string(),
                    loading: loading.clone(),
                }
            }

            // Footer
            AuthFooterButton {
                text: "Already have an account? ".to_string(),
                link_text: "Log in here".to_string(),
                to: Route::Login {},
            }
        }
    )
}
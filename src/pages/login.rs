use dioxus::prelude::*;

use crate::{backend::auth::{LoginRequest, login}, components::{button::{AuthButton, AuthFooterButton}, card::AuthCard, input::{PasswordInput, TextInput}, toast::{ToastContext, ToastKind}}, routes::Route};

#[component]
pub fn Login() -> Element {
    // let mut email: Signal<String> = use_signal(|| String::new());
    // let mut password: Signal<String> = use_signal(|| String::new());
    let mut email: Signal<String> = use_signal(|| "admin@some.mail".to_string());
    let mut password: Signal<String> = use_signal(|| "password".to_string());
    let mut loading: Signal<bool> = use_signal(|| false);
    let mut toast_ctx: ToastContext = use_context::<ToastContext>();

    let nav: dioxus_router::Navigator = use_navigator();

    rsx! {
        AuthCard { header: Some("Sign in to your account".to_string()),
            // Form
            form {
                class: "space-y-4",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();

                    loading.set(true);
                    match login(LoginRequest {
                            email: email(),
                            password: password(),
                        })
                        .await
                    {
                        Ok(_) => {
                            toast_ctx.push("Login successful".to_string(), ToastKind::Success);
                            info!("Login successful");
                            nav.push(Route::Dashboard {});
                        }
                        // why it goes here?
                        Err(e) => {
                            toast_ctx
                                .push(
                                    format!(
                                        "Login failed: {}",
                                        e.message.as_deref().unwrap_or("Something went wrong"),
                                    ),
                                    ToastKind::Error,
                                );
                            info!("Login failed: {:?}", e);
                        }
                    }
                    loading.set(false);
                },

                // Email
                TextInput {
                    value: email(),
                    oninput: move |val| email.set(val),
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

                    Link {
                        to: Route::ForgetPassword {},
                        class: "text-sm text-blue-600 hover:underline",
                        "Forgot password?"
                    }
                }

                // Submit
                AuthButton {
                    disabled: false,
                    label: "Sign In".to_string(),
                    loading: loading.clone(),
                }
            }

            // Footer
            AuthFooterButton {
                text: "Don’t have an account? ".to_string(),
                link_text: "Sign up".to_string(),
                to: Route::Register {},
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::{card::AuthCard, input::TextInput};
use crate::routes::Route;

#[component]
pub fn ForgetPassword() -> Element {
    rsx! {
        AuthCard { header: Some("Reset your password".to_string()),
            form { class: "space-y-4",
                TextInput {
                    label: "Email Address".to_string(),
                    name: "email".to_string(),
                    id: "email".to_string(),
                    value: "".to_string(),
                    oninput: |_| {},
                }

                button {
                    r#type: "submit",
                    class: "
                        w-full py-2 px-4
                        bg-blue-600 hover:bg-blue-700
                        text-white font-semibold
                        rounded-lg shadow-md
                        focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                    ",
                    "Send Reset Link"
                }

                p { class: "mt-6 text-center text-sm text-gray-600 dark:text-gray-400",
                    "Already have an account? "
                    Link {
                        to: Route::Login {},
                        class: "text-blue-600 hover:underline font-medium",
                        "Back to Login"
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct PasswordInputProps {
    #[props(optional)]
    pub value: String,

    #[props(optional)]
    pub oninput: EventHandler<String>,

    #[props(optional)]
    pub required: bool,

    #[props(optional)]
    pub placeholder: String,

    #[props(optional)]
    pub r#type: String,

    #[props(optional)]
    pub is_password_shown: Option<bool>,

    #[props(optional)]
    pub password_hidden: Option<bool>,

    #[props(optional)]
    pub error: Option<String>,

    pub name: String,
    pub label: String,
    pub id: String,
}

#[component]
pub fn PasswordInput(props: PasswordInputProps) -> Element {
    let mut show_password: Signal<bool> = use_signal(|| props.is_password_shown.unwrap_or(false));

    rsx! {
        Fragment {
            label { class: "block mb-1 text-sm font-medium text-gray-700 dark:text-gray-300",
                {props.label}
            }

            div { class: "relative",
                input {
                    name: props.name,
                    id: props.id,
                    value: props.value,
                    oninput: move |e| props.oninput.call(e.value().clone()),
                    r#type: if show_password() { "text" } else { "password" },
                    placeholder: props.placeholder,
                    required: props.required,
                    class: "
                        w-full px-3 py-2 pr-14
                        rounded-md
                        border border-gray-300 dark:border-gray-600
                        bg-white dark:bg-gray-700
                        text-gray-900 dark:text-white
                        focus:outline-none focus:ring-2 focus:ring-blue-500
                    ",
                }

                if props.password_hidden.unwrap_or(true) {
                    button {
                        r#type: "button",
                        onclick: move |_| show_password.toggle(),
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

                if props.error.is_some() {
                    p { class: "mt-1 text-sm text-red-600 dark:text-red-400", {props.error} }
                }
            }
        }
    }
}
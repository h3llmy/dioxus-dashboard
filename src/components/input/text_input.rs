use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TextInputProps {
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
    pub error: Option<String>,

    pub name: String,
    pub label: String,
    pub id: String,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    rsx! {
        Fragment {
            label { class: "block mb-1 text-sm font-medium text-gray-700 dark:text-gray-300",
                {props.label}
            }
            input {
                name: props.name,
                id: props.id,
                r#type: props.r#type,
                value: props.value,
                oninput: move |e| props.oninput.call(e.value().clone()),
                placeholder: props.placeholder,
                required: props.required,
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

            if props.error.is_some() {
                p { class: "mt-1 text-sm text-red-600 dark:text-red-400", {props.error} }
            }
        }
    }
}
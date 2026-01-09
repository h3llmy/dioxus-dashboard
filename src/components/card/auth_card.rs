use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct AuthCardProps {
    pub children: Element,

    #[props(optional)]
    pub header: Option<String>,
}

#[component]
pub fn AuthCard(props: AuthCardProps) -> Element {
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

                if props.header.is_some() {
                    // Header
                    div { class: "mb-10 text-center",
                        h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                            {props.header.clone().unwrap()}
                        }
                    }
                }

                {props.children}
            }
        }
    }
}
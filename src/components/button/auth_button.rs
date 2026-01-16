use dioxus::prelude::*;

#[component]
pub fn AuthButton(disabled: bool, label: String, loading: Signal<bool>) -> Element {
    rsx! {
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
}
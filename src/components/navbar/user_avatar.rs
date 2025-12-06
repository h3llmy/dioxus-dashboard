use dioxus::prelude::*;

#[component]
pub fn UserAvatar(user_menu_open: Signal<bool>) -> Element {
    rsx! {
        button {
            onclick: move |_| user_menu_open.toggle(),
            class: "flex text-sm bg-gray-800 rounded-full cursor-pointer focus:ring-4
                    focus:ring-gray-300 dark:focus:ring-gray-600",
            r#type: "button",

            span { class: "sr-only", "Open user menu" }

            img {
                alt: "user photo",
                class: "w-8 h-8 rounded-full cursor-pointer",
                src: "https://flowbite.com/docs/images/people/profile-picture-5.jpg",
            }
        }
    }
}

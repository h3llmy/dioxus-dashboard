use dioxus::prelude::*;

#[component]
pub fn NavbarLogo() -> Element {
    rsx! {
        a {
            class: "flex ms-2 md:me-24",
            href: "https://flowbite.com",

            img {
                alt: "FlowBite Logo",
                class: "h-8 me-3",
                src: "https://flowbite.com/docs/images/logo.svg",
            }

            span {
                class: "self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white",
                "Flowbite"
            }
        }
    }
}

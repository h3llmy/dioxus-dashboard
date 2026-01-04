use dioxus::prelude::*;
use dioxus_sdk_storage::{LocalStorage, use_synced_storage};
use web_sys::{HtmlElement, window};
use wasm_bindgen::JsCast;

pub fn use_dark_mode() -> Signal<bool> {
    let dark: Signal<bool> = use_synced_storage::<LocalStorage, bool>("theme".to_string(), || false);

    use_effect({
        let dark: Signal<bool> = dark.clone();
        move || {
            let document = window().unwrap().document().unwrap();
            let html: HtmlElement = document.document_element().unwrap().unchecked_into();
            html.set_class_name(if dark() { "dark" } else { "" });
        }
    });
    dark
}
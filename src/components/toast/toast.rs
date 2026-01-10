use dioxus::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id: Uuid,
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Props, Clone, PartialEq)]
struct ToastItemProps {
    toast: ToastData,
    on_remove: EventHandler<Uuid>,
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    pub toasts: Signal<Vec<ToastData>>,
}

impl ToastContext {
    pub fn push(&mut self, message: String, kind: ToastKind) {
        let id = Uuid::new_v4();
        self.toasts.write().push(ToastData { id, message, kind });
    }

    pub fn remove(&mut self, id: Uuid) {
        self.toasts.write().retain(|t| t.id != id);
    }
}

#[derive(PartialEq, Clone)]
pub enum ToastKind { Info, Success, Error, Warning }

impl ToastKind {
    pub fn classes(&self) -> &'static str {
        match self {
            ToastKind::Info => "bg-blue-50 border-blue-200 text-blue-800",
            ToastKind::Success => "bg-green-50 border-green-200 text-green-800",
            ToastKind::Error => "bg-red-50 border-red-200 text-red-800",
            ToastKind::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
        }
    }
}

#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toasts = use_signal(Vec::<ToastData>::new);
    let mut context = ToastContext { toasts };
    use_context_provider(|| context);

    rsx! {
        // Global styles for the animation
        style {
            "@keyframes toast-in {{
                from {{ transform: translateX(100%); opacity: 0; }}
                to {{ transform: translateX(0); opacity: 1; }}
            }}"
        }

        {children}

        // Multiple toast stack container
        div { class: "fixed top-4 right-4 z-[100] flex flex-col gap-2 pointer-events-none",
            for toast in toasts.read().iter() {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    on_remove: move |id| context.remove(id),
                }
            }
        }
    }
}
#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let toast_id = props.toast.id;

    // Auto-dismiss after 5 seconds (WASM SAFE)
    use_effect(move || {
        spawn(async move {
            sleep(Duration::from_secs(5)).await;
            props.on_remove.call(toast_id);
        });
    });

    rsx! {
        div {
            class: "pointer-events-auto",
            style: "animation: toast-in 0.3s ease-out forwards;",
            div {
                class: format!(
                    "w-72 p-4 rounded-lg shadow-lg border flex justify-between items-center {}",
                    props.toast.kind.classes(),
                ),
                span { class: "text-sm font-medium", "{props.toast.message}" }
                button {
                    class: "ml-4 opacity-70 hover:opacity-100 transition-opacity",
                    onclick: move |_| props.on_remove.call(toast_id),
                    "✕"
                }
            }
        }
    }
}
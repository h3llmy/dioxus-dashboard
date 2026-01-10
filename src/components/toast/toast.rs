use dioxus::prelude::*;

#[derive(Clone)]
pub struct ToastData {
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Clone)]
pub struct ToastContext {
    pub toasts: Signal<Vec<ToastData>>,
}

#[derive(PartialEq, Clone)]
pub enum ToastKind {
    Info,
    Success,
    Error,
    Warning,
}

impl ToastKind {
    pub fn classes(&self) -> &'static str {
        match self {
            ToastKind::Info => "bg-blue-50 border border-blue-200 text-blue-800",
            ToastKind::Success => "bg-green-50 border border-green-200 text-green-800",
            ToastKind::Error => "bg-red-50 border border-red-200 text-red-800",
            ToastKind::Warning => "bg-yellow-50 border border-yellow-200 text-yellow-800",
        }
    }
}


#[component]
pub fn ToastProvider(children: Element) -> Element {
    // create a signal to hold active toasts
    let mut toasts: Signal<Vec<ToastData>> = use_signal(|| Vec::<ToastData>::new());

    // provide it to descendants
    use_context_provider(|| ToastContext {
        toasts: toasts.clone(),
    });

    rsx! {
        {children}

        // render each toast
        for (i , toast) in toasts.read().iter().enumerate() {
            div { key: "{i}", class: "fixed top-4 right-4 z-50",
                div { class: format!("max-w-sm w-full p-3 rounded-lg shadow-lg {}", toast.kind.classes()),
                    div { class: "flex justify-between items-center space-x-4",
                        span { "{toast.message}" }
                        button {
                            onclick: move |_| {
                                // remove this toast
                                toasts.write().remove(i);
                            },
                            "X"
                        }
                    }
                }
            }
        }
    }
}

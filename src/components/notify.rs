use dioxus::prelude::*;

pub struct NotifyOptions {
    text: Option<String>,
    timeout_ms: usize,
}

impl NotifyOptions {
    pub fn new(text: String, timeout_ms: usize) -> Self {
        Self {
            text: Some(text),
            timeout_ms,
        }
    }
}

static NOTIFY_OPTIONS: GlobalSignal<NotifyOptions> =
    Global::new(|| NotifyOptions::new("".into(), 0));

#[component]
pub fn Notify() -> Element {
    rsx! {
        div { class: "notification pointer-events-none fixed top-3 right-3 flex border-[#d08770] border-2 w-fit min-w-70 max-h-25 py-1",
            img { class: "mx-1 w-20 my-auto", src: asset!("/assets/images/mokou.avif") }
            div { class: "flex flex-col",
                span { class: "text-2xl", " 藤原妹红: " }
                span { class: "notification-content text-xl my-auto whitespace-pre" }
            }
        }
    }
}

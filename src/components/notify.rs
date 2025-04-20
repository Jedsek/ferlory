use dioxus::prelude::*;
use gloo::timers::callback::Timeout;

pub struct State {
    pub content: &'static str,
    pub hidden: bool,
    pub timeout_handle: Option<Timeout>,
}

pub static STATE: GlobalSignal<State> =
    Global::new(|| State {
        content: "",
        hidden: true,
        timeout_handle: None,
    });

#[component]
pub fn Notification() -> Element {
    let content = STATE.read().content;
    
    rsx! {
        div {
            hidden: STATE.read().hidden,
            class: "invisible xl:visible",
            class: "flex flex-row",
            class: "border-l-3 bg-slate-800 pointer-events-none fixed! top-6 right-6 w-fit min-w-70 max-h-25 py-1",
            div { class: "px-2 flex flex-col justify-content-end",
                span { class: "text-2xl italic font-semibold", "藤原妹红:" }
                span { class: "text-xl my-auto whitespace-pre", {content} }
            }
            img { class: " border-warm-orange border-2 mx-1 w-25 my-auto", src: "/assets/images/mokou.avif" }
        }
    }
}

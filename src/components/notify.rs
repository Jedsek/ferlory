use dioxus::prelude::*;
use gloo::timers::callback::Timeout;

pub struct State {
    pub content: String,
    pub hidden: bool,
    pub timeout_handle: Option<Timeout>,
}

pub static STATE: GlobalSignal<State> =
    Global::new(|| State {
        content: "".into(),
        hidden: true,
        timeout_handle: None,
    });

#[component]
pub fn Notification() -> Element {
    let opacity = if STATE.read().hidden { "opacity-0!" } else { "opacity-100!" };
    rsx! {
        div {
            class: "invisible! lg:visible! lg:flex lg:flex-row {opacity}",
            class: "border-l-3 bg-slate-800 pointer-events-none fixed! top-6 right-6 w-fit min-w-70 max-h-25 py-1",
            class: "transition-opacity duration-300 ease-in-out",
            div { class: "px-2 flex flex-col justify-content-end",
                span { class: "text-2xl italic bold", " 藤原妹红:" }
                span { class: "text-xl my-auto whitespace-pre", {STATE.read().content.clone()} }
            }
            img { class: " border-warm-orange border-2 mx-1 w-25 my-auto", src: asset!("/assets/images/mokou.avif") }
        }
    }
}

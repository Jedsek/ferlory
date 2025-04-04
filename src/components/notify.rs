use dioxus::{logger::tracing::{self, trace}, prelude::*};
use dioxus_elements::track;
use gloo::timers::callback::Timeout;

struct State {
    content: String,
    hidden: bool,
    timeout_ms: u32,
    timeout_handle: Option<Timeout>,
}

pub fn notify_send<T: Into<String>>(content: T, timeout_ms: Option<u32>) {
    STATE.write().hidden = false;
    STATE.write().content = content.into();
    match timeout_ms {
        Some(timeout_ms) =>  {
            STATE.write().timeout_handle = Some(Timeout::new(timeout_ms, || {
            STATE.write().hidden = true;
        }))},
        None => ()
    }

}

static STATE: GlobalSignal<State> =
    Global::new(|| State {
        content: "".into(),
        hidden: true,
        timeout_ms: 0,
        timeout_handle: None,
    });

#[component]
pub fn Notification() -> Element {
    let opacity = if STATE.read().hidden { "opacity-0" } else { "opacity-100" };
    rsx! {
        div { class: "{opacity} pointer-events-none fixed top-3 right-3 flex border-[#d08770] border-2 w-fit min-w-70 max-h-25 py-1",
            img { class: "mx-1 w-20 my-auto", src: asset!("/assets/images/mokou.avif") }
            div { class: "flex flex-col",
                span { class: "text-2xl", " 藤原妹红: " }
                span { class: "text-lg pr-2 my-auto whitespace-pre", {STATE.read().content.clone()} }
            }
        }
    }
}

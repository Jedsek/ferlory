use dioxus::prelude::*;

#[component]
pub fn Placeholder() -> Element {
    rsx! {
        div { dangerous_inner_html: "<div></div>" }
    }
}

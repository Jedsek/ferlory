use dioxus::prelude::*;

#[component]
pub fn SplitLine() -> Element {
    rsx! {
        div { class: "border-1 border-slate-400 my-4" }
    }
}

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "min-h-screen h-2000",
            // Content
            p { "In blog, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
            h1 { class: "text-2xl mb-60", id: "a", "a"}
            h1 { class: "text-2xl mb-60", id: "b", "b"}
            h1 { class: "text-2xl mb-60", id: "c", "c"}

        }
    }
}

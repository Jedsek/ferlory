use dioxus::prelude::*;

#[component]
pub fn ErrorPage() -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            p { "In blog, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

        }
    }
}

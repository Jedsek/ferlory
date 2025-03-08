use crate::components::Avatar;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Avatar {},
        div { dangerous_inner_html: include_str!("../../assets/content/home.html") }

    }
}

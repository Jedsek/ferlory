use dioxus::prelude::*;
use crate::components::post_components::*;

#[component]
pub fn Programming() -> Element {
    rsx! {
        blockquote { "本页面记载一些计算机方面的技术文章" }
        PostCard { serioes: "rust-tui", "rust-tui" }
        TypstPost { path: "home.typ" }
    }
}

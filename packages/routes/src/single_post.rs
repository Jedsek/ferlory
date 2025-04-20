use dioxus::prelude::*;

#[component]
pub fn SinglePost(name: String) -> Element {
    rsx! {
        blockquote { "喜悦, 悲伤, 记载于此" }
    }
}


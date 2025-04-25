use dioxus::prelude::*;

#[component]
pub fn SeriesPost(series: String, name: String) -> Element {
    rsx! {
        blockquote { "喜悦, 悲伤, 记载于此" }
    }
}

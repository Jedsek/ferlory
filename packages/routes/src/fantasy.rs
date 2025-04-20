use dioxus::prelude::*;

#[component]
pub fn Fantasy() -> Element {
    rsx! {
        blockquote { "苦苦追寻不存于世之美好\n对与世隔绝之幻想, 抱以无上热忱! !" }
    }
}

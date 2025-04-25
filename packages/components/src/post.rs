use dioxus::prelude::*;

#[component]
pub fn TypstPost(path: &'static str) -> Element {
    let content = generated_typst::get(path);
    rsx! {
        div { class: "",
            dangerous_inner_html: content
        }
    }
}

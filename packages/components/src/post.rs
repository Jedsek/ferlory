use dioxus::prelude::*;

#[component]
pub fn TypstPost(path: &'static str) -> Element {
    let content = generated_typst::CONTENTS.get(path).unwrap().to_owned();
    rsx! {
        div { class: "",
            dangerous_inner_html: content
        }
    }
}

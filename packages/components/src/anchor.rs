use dioxus::prelude::*;
use slug::slugify;

#[component]
pub fn H1(id: Option<&'static str>, text: &'static str) -> Element {
    let id = slugify(id.unwrap_or(text));
    let id = id.as_str();
    rsx! {
        h1 { id, Link { to: format!("#{id}"), {text} } }
    }
}

#[component]
pub fn H2(id: Option<&'static str>, text: &'static str) -> Element {
    let id = slugify(id.unwrap_or(text));
    let id = id.as_str();
    rsx! {
        h2 { id, Link { to: format!("#{id}"), {text} } }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Friends() -> Element {
    rsx! {
        div {
            blockquote { "有朋自远方来, 不亦乐乎" }
            Friend {
                name: "Amiriox",
                avatar: "https://amiriox.github.io/images/icon.jpg",
                href: "https://amiriox.github.io/"
            }
        }
    }
}


#[component]
pub fn Friend(name: &'static str, avatar: &'static str, href: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-row border-l-4 border-sky-500",
            img { class: "w-25 mx-3 my-4", src: avatar }
            div {
                class: "flex flex-col *:my-auto",
                span { class: "text-2xl! italic font-medium", "{name}" }
                a { href, "{href}" }
            }
        }
    }
}

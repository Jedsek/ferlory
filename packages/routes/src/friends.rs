use dioxus::prelude::*;

#[component]
pub fn Friends() -> Element {
    rsx! {
        div {
            blockquote { "有朋自远方来, 不亦乐乎" }

            div { class: "mx-1",
                Friend {
                    name: "折鸦夜明け前(Amiriox)",
                    avatar: "https://amiriox.github.io/images/icon.jpg",
                    href: "https://amiriox.github.io/"
                }
            }
        }
    }
}

#[component]
fn Friend(name: &'static str, avatar: &'static str, href: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-row border-l-4 border-sky-500 items-center",
            img { class: "w-14 mx-2 my-2", src: avatar }
            div {
                class: "flex flex-col",
                span { class: "text-lg! italic font-medium", "{name}" }
                a { href, "{href}" }
            }
        }
    }
}

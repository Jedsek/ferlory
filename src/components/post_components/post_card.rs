use dioxus::prelude::*;

#[component]
pub fn PostCard(serioes: Option<&'static str>, children: Element) -> Element {
    rsx! {
        Link {
            class: "inline-block no-underline!",
            to: "",
            div {
                class: "w-100! h-30! border-1 no-underline",
                {children}
            }
        }
    }
}


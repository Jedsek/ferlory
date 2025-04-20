use const_format::unwrap;
use dioxus::prelude::*;

#[component]
pub fn Details(summary: Element, hidden: Option<bool>, children: Element) -> Element {
    let hidden = hidden.unwrap_or(true);
    let mut hidden = use_signal(|| hidden);
    let is_hidden = *hidden.read();
    
    rsx!{
        div {
            class: "my-3",
            class: "transition-all duration-500 ease-in-out",
            div {
                class: "text-xl w-fit my-2",
                onclick: move |_data| *hidden.write() ^= true,
                i { class: "iconfont icon-arrow-down  text-xl!", class: if is_hidden { "hidden" } else { "inline" } }
                i { class: "iconfont icon-arrow-right text-xl!", class: if is_hidden { "inline" } else { "hidden" }}
                {summary}
            }
            ul {
                class: if is_hidden { "opacity-0 hidden" } else { "opacity-100 block" },
                class: "transition-all duration-500 ease-in-out",
                {children}
            }

        }
    }
}


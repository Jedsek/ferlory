use dioxus::prelude::*;

#[component]
pub fn Todo(completed: Option<bool>, children: Element) -> Element {
    let completed = completed.unwrap_or_default();

    rsx! {
        div {
            class: "my-1 flex flex-row",
            input {
                type: "checkbox",
                checked: completed,
                class: "w-5! h-5! text-4xl! mx-3 align-middle shrink-0",
                class: "appearance-none outline-1 outline-solid outline-sky-300",
                class: if completed { "after:inline-block after:content-['L'] after:text-cyan-200" },
                class: if completed { "after:-translate-x-[0.4px] after:-translate-y-[11px] after:rotate-40 after:-scale-x-65 after:scale-y-70" },
            }
            div {
                {children}
            }
        }
    }
}

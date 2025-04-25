use dioxus::prelude::*;

#[component]
pub fn ErrorPage() -> Element {
    let mut state = use_signal(|| None);
    let idx = *state.read();

    rsx! {
        div {
            class: "text-2xl w-fit mx-auto mt-2 sm:mt-4 *:mt-2",
            class: "sm:text-3xl",
            div {
                class: "w-fit mx-auto *:mx- *:inline-block",
                class: "*:even:text-slate-600 *:even:font-bold *:even:animate-ping",
                class: "sm:*:even:text-4xl sm:*:mx-2",
                span { class: if idx == Some(0) { "scale-120 font-bold text-red-500" } else { "text-slate-500" }, "术师" }
                span { "<|>" }
                span { class: if idx == Some(1) { "scale-120 font-bold text-red-500" } else { "text-slate-500" }, "皇帝" }
                span { "<|>" }
                span { class: if idx == Some(2) { "scale-120 font-bold text-red-500" } else { "text-slate-500" }, "力量" }
            }
            div { class: "w-fit mx-auto text-red-500 text-xl font-semibold",
                "你擅自闯入了不该进入的地方"
            }
            div { class: "w-fit mx-auto text-red-500 text-xl font-semibold animate-bounce",
                "『期望之物』不曾存在"
            }

        }
        div {
            class: "flex flex-col w-fit mx-auto",
            class: "sm:flex-row",
            for (idx, name) in ["magician", "emperor", "strength"].into_iter().enumerate() {
                img {
                    class: "m-4 sm:w-1/3 sm:max-h-[65vh] hover:scale-105 transition duration-200 ease-in-out",
                    onmouseenter: move |_data| *state.write() = Some(idx),
                    onmouseleave: move |_data| *state.write() = None,
                    src: format!("/assets/images/arcana/ghost/{name}.avif")
                }
            }
        }
    }
}

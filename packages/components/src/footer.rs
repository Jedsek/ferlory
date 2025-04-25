use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "h-[50vh] *:px-auto border-t-2 mt-[50vh] pt-5",
            class: "flex flex-col text-center",
            div {
                // i { class: "iconfont icon-love text-red-500! bg-clip-text bg-red-500 align-middle! text-3xl!" }
                span { class: "text-2xl opacity-90", "柳下川 <|> Jedsek <|> 清贫" }
            }
            "本博客均采用 \"CC BY-NC-ND 4.0\" 协议"
        }
    }
}

#![allow(non_upper_case_globals)]

use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    // const _A: Asset = asset!("/assets/content/home.html");
    // let dir = include_dir!("$CARGO_MANIFEST_DIR/target/dx/ferlora/debug/web/public/assets");
    // let a = dir.get_file(_A.resolve().strip_prefix("/assets").to_str().unwrap()).unwrap();

    rsx! {
        div {
            class: "h-fit flex flex-col mt-8",
            class: "sm:flex-row sm:ml-6 sm:my-8",
            Avatar {}
            Info {}
            Navigation {}
            // div { dangerous_inner_html: include_str!("../../assets/content/home.html") }
        }
    }
}

#[component]
fn Avatar() -> Element {
    const AVATAR: Asset = asset!("/assets/images/avatar.avif");
    const AVATAR_BORDER: Asset = asset!("/assets/images/avatar_border.avif");

    rsx! {
        div {
            class: "flex justify-center h-fit hover:animate-bounce",
            class: "sm:justify-start ",
            a {
                class: "w-50 sm:w-60 flex justify-center items-center relative group",
                class: "duration-200 hover:scale-110 sm:hover:scale-90 hover:rotate-12 sm:hover:-rotate-12 hover:opacity-80",
                href: "/programming",
                img { class: "w-40 sm:w-50", src: "{AVATAR}"  }
                img { class: "w-50 sm:w-60 absolute", src: "{AVATAR_BORDER}"  }
                i { class: "text-6xl! text-black opacity-60 iconfont icon-restart absolute hidden group-hover:inline" }
            }
        }
    }
}

#[component]
fn Info() -> Element {
    rsx! {
        div {
            class: "group h-fit mt-10 pt-4 border-t-2 border-dashed border-sky-400",
            class: "sm:mt-0 sm:py-0 sm:ml-10 sm:border-l-3 sm:border-y-0",
            p {
                class: "animate-bounce duration-100 w-fit mx-auto sm:ml-3 sm:py-3 text-4xl! font-semibold",
                class: "bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                "柳下川"
            }
            div {
                class: "sm:mx-0 sm:pt-3 sm:h-fit *:text-lg! *:italic *:text-slate-400",
                class: "*:[&p_span]:last:hover:underline",
                p { span { "-->" } span { "与其浊富, 宁比清贫" } }
                p { span { "~~>" } span { "cosplay堂吉珂德" } }
                p { span { "==>" } span { "加缪式钢铁活法" } }
                p { span { ">>>" } span { "galgame!!!" } }
            }
        }
    }
}

#[component]
fn Navigation() -> Element {
    let mut text = use_signal(|| rsx! {""});

    rsx! {
        div {
            class: "group h-fit my-4 pt-2 pb-4 border-y-2 border-dashed border-purple-400",
            class: "sm:my-0 sm:py-0 sm:mx-10 sm:border-l-3 sm:border-y-0",
            p {
                class: "duration-100 w-fit mx-auto sm:ml-3 sm:py-3 text-4xl! font-semibold",
                span {
                    class: "bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                    span { "GOTO" }
                    span { class: "hidden group-hover:inline", ">> " }
                }
                span { class: "hidden italic text-fuchsia-400 sm:group-hover:inline", {text} }
            }
            div {
                class: "grid grid-rows-2 grid-flow-col w-fit mx-auto *:m-1",
                class: "sm:ml-4",
                class: "*:border-2 *:border-sky-600 *:rounded-sm",
                class: "*:text-2xl! *:flex *:flex-row *:items-center *:justify-center *:p-1",
                class: "*:[&div_i]:mx-1 *:[&div_i]:text-3xl! *:[&div_i]:group-hover:animate-spin",
                class: "*:hover:scale-110 *:hover:opacity-70 *:hover:duration-200 *:odd:hover:-translate-y-2 *:even:hover:translate-y-2",
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{"/home"}) },
                    i { class: "iconfont icon-home" }
                    Link { to: Route::Home {}, "首页" }
                }
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{"/programming"}) },
                    i { class: "iconfont icon-code" }
                    Link { to: Route::Programming {}, "编程" }
                }
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{"/about"}) },
                    i { class: "iconfont icon-about" }
                    Link { to: Route::About {}, "关于" }
                }
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{"/fantasy"}) },
                    i { class: "iconfont icon-flower" }
                    Link { to: Route::Fantasy {}, "幻想" }
                }
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{"/friends"}) },
                    i { class: "iconfont icon-love" }
                    Link { to: Route::Friends {}, "友链" }
                }
                div { class: "group", onmouseover: move |_data| { text.set(rsx!{del {"/error"}}) },
                    i { class: "iconfont icon-error" }
                    Link { to: Route::ErrorPage {}, del { "错误" } }
                }
            }
        }
    }
}

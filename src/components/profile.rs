#![allow(non_upper_case_globals)]

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
            class: "flex justify-center h-fit",
            class: "sm:justify-start ",
            div {
                class: "w-50 sm:w-60 flex justify-center items-center relative",
                class: "duration-400 hover:scale-110 sm:hover:scale-90 hover:rotate-12 sm:hover:-rotate-12 hover:opacity-70",
                img { class: "w-40 sm:w-50", src: "{AVATAR}"  }
                img { class: "w-50 sm:w-60 absolute", src: "{AVATAR_BORDER}"  }
            }
        }
    }
}

#[component]
fn Info() -> Element {
    rsx! {
        div {
            class: "h-fit mt-10 pt-4 border-t-2 border-dashed border-sky-400",
            class: "sm:mt-0 sm:py-0 sm:ml-10 sm:border-l-3 sm:border-y-0",
            p { class: "w-fit mx-auto sm:ml-3 sm:py-3 text-4xl font-semibold bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                "柳下川"
            }
            div { class: "sm:mx-2 sm:pt-3 sm:h-fit *:italic *:text-slate-500",
                p { "-> 与其浊富, 宁比清贫" }
                p { "~> cosplay堂吉珂德" }
                p { "=> 加缪式钢铁活法" }
                p { ">> galgame!!!" }
            }
        }
    }
}

#[component]
fn Navigation() -> Element {
    rsx! {
        div {
            class: "h-fit my-4 pt-4 border-y-2 border-dashed border-purple-400",
            class: "sm:my-0 sm:py-0 sm:mx-10 sm:border-l-3 sm:border-y-0",
            p { class: "w-fit mx-auto sm:ml-3 sm:py-3 text-4xl font-semibold bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                span { "GOTO" }
                span { class: "hidden sm:inline", "~>" }
            }
            div {
                class: "grid grid-flow-col grid-rows-2 gap-4 mb-4 sm:mb-0 justify-between h-fit",
                class: "*:w-fit *:mx-10 *:text-2xl *:border-1 *:border-purple-500",
                // class: "sm:mx-10",
                div { "首页" }
                div { "编程" }
                div { "幻想" }
                div { "错误" }
            }
        }
    }
}

use dioxus::prelude::*;
use crate::utils::notify_send;
use crate::route::Route;

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
    rsx! {
        div {
            class: "flex justify-center h-fit hover:animate-bounce",
            class: "sm:justify-start ",
            a {
                href: "/about",
                class: "w-50 sm:w-60 flex justify-center items-center relative group",
                class: "duration-200 hover:scale-110 sm:hover:scale-90 hover:rotate-12 sm:hover:-rotate-12 hover:opacity-80",
                img { class: "w-40 sm:w-50", src: asset!("/assets/images/avatar.avif") }
                img { class: "w-50 sm:w-60 absolute", src: asset!("/assets/images/avatar_border.avif") }
                i { class: "text-6xl! text-black opacity-60 iconfont icon-restart absolute hidden group-hover:inline animate-spin" }
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
            div {
                class: "animate-bounce duration-100 w-fit mx-auto sm:ml-3 sm:py-3 text-4xl! font-semibold",
                class: "bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                "柳下川"
            }
            div {
                class: "sm:mx-0 sm:pt-3 sm:h-fit *:text-lg! *:italic *:text-slate-400",
                div { span { "--> " } a { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", href: "/about/#清贫", "与其浊富, 宁比清贫" } }
                div { span { "==> " } a { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", href: "/about/#堂吉柯德", "cosplay堂吉珂德" } }
                div { span { "=== " } a { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", href: "/about/#加缪式钢铁活法", "加缪式钢铁活法" } }
                div { span { ">>> " } a { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", href: "/about/#galgame", "galgame!!!" } }
            }
        }
    }
}

#[component]
fn Navigation() -> Element {
    let mut state: Signal<Option<&'static str>> = use_signal(|| None);
    use_context_provider(|| state);

    rsx! {
        div {
            class: "h-fit my-4 pt-2 pb-4 border-y-2 border-dashed border-purple-400",
            class: "sm:my-0 sm:py-0 sm:mx-10 sm:border-l-3 sm:border-y-0",
            div { class: "duration-100 w-fit mx-auto sm:ml-3 sm:py-3 text-4xl! font-semibold",
                span { class: "bg-gradient-to-r bg-clip-text from-sky-400/90 to-purple-600/100 text-transparent",
                    match *state.read() {
                        None => rsx! { "GOTO" },
                        Some(text) => rsx! {
                            ">> ",
                            span { class: "italic text-fuchsia-400",
                                if text == "/error" { del { "{text}" } } else { "{text}" }
                            }
                        },
                    }
                }
            }
            div {
                onmouseleave: move |_data| state.set(None),
                class: "grid grid-rows-2 grid-flow-col w-fit mx-auto *:m-1",
                class: "sm:ml-4",
                class: "*:border-2 *:border-sky-600 *:rounded-sm **:no-underline! **:not-italic! **:font-normal!",
                class: "*:text-2xl! *:flex *:flex-row *:items-center *:justify-center *:p-1",
                class: "**:[a_i]:mx-1 **:[a_i]:text-2xl!",
                class: "*:hover:scale-110 *:hover:opacity-70 *:hover:duration-200 *:odd:hover:-translate-y-2 *:even:hover:translate-y-2",
                NavigationItem {
                    route: Route::Moments {},
                    tip_text: "那是段很让人怀念的时光呢",
                    i { class: "iconfont icon-chat text-sky-500" }
                }
                NavigationItem {
                    route: Route::Programming {},
                    tip_text: "别指望我能帮你什么...\n自己解决!",
                    i { class: "iconfont icon-code text-sky-500" }
                }
                NavigationItem {
                    route: Route::About {},
                    tip_text: "哦? 原来你————",
                    i { class: "iconfont icon-about text-sky-500" }
                }
                NavigationItem {
                    route: Route::Fantasy {},
                    tip_text: "有点意思, 嗯, 不错",
                    i { class: "iconfont icon-flower text-sky-500" }
                }
                NavigationItem {
                    route: Route::Friends {},
                    tip_text: "喂, 这次我来请你喝酒\n就当是上一次的回礼吧",
                    i { class: "iconfont icon-love text-red-400" }
                }
                NavigationItem {
                    route: Route::ErrorPage {},
                    tip_text: "? 好███怪的感觉, ██████\n等等...快███回来, ███险!!!",
                    i { class: "iconfont icon-error text-slate-500" }
                }
            }
        }
    }
}

#[component]
pub fn NavigationItem(route: Route, tip_text: &'static str, children: Element) -> Element {
    let mut state = use_context::<Signal<Option<&'static str>>>();
    
    rsx!{
        div {
            onmouseenter: move |_data| {
                state.set(Some(route.route_name()));
                notify_send(Some(tip_text), None);
            },
            onmouseleave: move |_data| notify_send(None, Some(0)),
            onclick: move |_data| notify_send(None, Some(1500)),
            Link { to: route,
                {children}
                if route.english_name() == "error" {
                    del { "{route.chinese_name()}" }
                } else {
                    "{route.chinese_name()}"
                }
            }
        }
    }
}

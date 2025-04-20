use dioxus::prelude::*;
use crate::components::post_components::*;
use crate::route::Route;
use crate::utils::notify_send;

#[component]
pub fn Home() -> Element {
    rsx! {
        Profile {}

        div {
            div { class: "mb-6",
                div { "=> 随意探索吧!! " "可能会有" span { class: "font-bold", " 惊喜 " } "哦~" }
                div { class: "flex flex-col sm:flex-row sm:*:mx-2",
                    "=> 这个博客本身就是一个 rust 项目: "
                    a { href: "https://github.com/jedsek/ferlory", "https://github.com/jedsek/ferlory" }
                }
            }

            Details {
                summary: rsx! { span { class: "text-sky-500", "我是谁?" } },
                li { "网名有 Jedsek, 柳下川, 清贫 等" }
                li { "爱好 ACGN(Animation <=> Comic <=> Game <=> Novel), 游戏偏好 2D, 如 SANABI(闪避刺客), Hollow-knights(空洞骑士)等" }
                li { "追寻艺术之人, 兴趣使然之人, 沉溺幻想之人, 拥抱孤独之人, 堕入虚无之人, 热爱世界之人" }
                li { "熟人面前放肆无比, 陌生人面前唯唯诺诺, 讨厌人挤人的环境(比如漫展), 是传统向宅男" }
                li { "东方Project中, 我推的角色是 依神紫菀(我头像就是这位哦, 她可爱吧?!)" }
                li { "即使感到迷茫也要前行, 我们都是......迷途之子啊! (?" }
            }

            Details {
                summary: rsx! { span { class: "text-pink-500", "博客介绍" } },
                li { "这里是我的个人博客, 内容包括计算机方面的文章, 自己写的小说, 一些 galgame 中的语录, 乱七八糟的想法等" }
                li { "本站使用 dioxus 构建, 迁移历史为 hexo-next -> zola -> dioxus " del { "(做到了 pure rust/wasm && no javascript)" } }
                li { del { "手机/平板上有些地方可能不怎么适配 (因为我也前端恐惧症, 而且......我懒得改啦!!!)" } }
                li { "手机/平板上已经进行适配, 但为了最佳体验请用电脑查看喔!" }
            }

            Details {
                summary: rsx!{ span { class: "text-slate-500", "乱七八糟的想法与吐槽" } },
                li { "受不了了, css 好难, 我一顿乱糊结果居然 ok 了, 而且标准也很杂乱啊啊啊啊" }
                li { "麻麻的, 高中牲果然不是人, 大好的国庆, 大好的假期, 3天, 噫! 3天! 哈哈呵呵呵呵哈哈哈哈哈哈" }
                li { "本站还在飞速优化优化inging!!! 等我看完小说打会音游吃掉手里的串串就打开电脑敲代...等等, 新番更新日?! 受不鸟了" }
                li { "又是被 rust 生命周期暴打, 被编译器爸爸摁在地上锤的一天" }
            }

            div { class: "min-h-[50vh]",
                div { class: "text-xl mt-8 mb-4", "博客构建的 TODO 列表:" }
                Todo { completed: true, "在较大屏幕上显示右上角的弹幕" }
                Todo { completed: true, "从 tailwind-v3 迁移至 tailwind-v4" }
                Todo { completed: true, "适配移动端适配, 优化手机上的体验" }
                Todo { completed: false, "静态的代码高亮, 去除 highlight.js" }
                Todo { completed: false, "类似 " a { href: "https://shaunlebron.github.io/parinfer/", "parinfer" } " 的方案与 lisp/scheme 代码进行交互" }
                Todo { completed: false, "清除史山与重构代码 " del { "(遥遥无期是也)" } }
                
            }
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        div {
            class: "h-fit flex flex-col mt-8",
            class: "sm:flex-row sm:ml-6 sm:my-8",
            Avatar {}
            Info {}
            Navigation {}
        }
    }
}

#[component]
fn Avatar() -> Element {
    rsx! {
        div {
            class: "flex justify-center h-fit hover:animate-bounce",
            class: "sm:justify-start ",
            Link {
                to: Route::About {},
                class: "w-50 sm:w-60 flex justify-center items-center relative group duration-200 hover:scale-110 sm:hover:scale-90 hover:rotate-12 sm:hover:-rotate-12 hover:opacity-80",
                img { class: "w-40 sm:w-50", src: "/assets/images/avatar.avif" }
                img { class: "w-50 sm:w-60 absolute", src: "/assets/images/avatar_border.avif" }
                i { class: "text-6xl! text-black opacity-60 iconfont icon-restart absolute invisible group-hover:visible animate-spin" }
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
                div { span { "~~> " } Link { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", to: "/about#ta-luo-pai", "诉说吧!『阿尔卡纳』" } }
                div { span { "--> " } Link { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", to: "/about#qing-pin", "与其浊富, 宁比清贫" } }
                div { span { "==> " } Link { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", to: "/about/#gang-tie-yi-zhi", "cosplay堂吉珂德" } }
                div { span { "=== " } Link { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", to: "/about/#gang-tie-yi-zhi", "加缪式钢铁活法" } }
                div { span { ">>> " } Link { class: "no-underline! hover:underline! hover:font-semibold! font-normal!", to: "/about/#galgame", "galgame!!!" } }
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
                    route: Route::Programming {},
                    tip: "别指望我能帮你什么...\n自己解决!",
                    icon: rsx!{ i { class: "iconfont icon-code text-sky-500" } }
                }
                NavigationItem {
                    route: Route::Moments {},
                    tip: "那是段很让人怀念的时光呢",
                    icon: rsx!{ i { class: "iconfont icon-chat text-sky-500" } }
                }
                NavigationItem {
                    route: Route::About {},
                    tip: "哦? 原来你————",
                    icon: rsx!{ i { class: "iconfont icon-about text-sky-500" } }
                }
                NavigationItem {
                    route: Route::Fantasy {},
                    tip: "有点意思, 嗯, 不错",
                    icon: rsx!{ i { class: "iconfont icon-flower text-sky-500" } }
                }
                NavigationItem {
                    route: Route::Friends {},
                    tip: "喂, 这次我来请你喝酒\n就当是上一次的回礼吧",
                    icon: rsx!{ i { class: "iconfont icon-love text-red-400" } }
                }
                NavigationItem {
                    route: Route::ErrorPage {},
                    tip: "? ! ! ████那是████████████\n等等...快███回来, ███险! !",
                    icon: rsx!{ i { class: "iconfont icon-error text-slate-500" } }
                }
            }
        }
    }
}

#[component]
fn NavigationItem(route: Route, tip: &'static str, icon: Element) -> Element {
    let mut state = use_context::<Signal<Option<&'static str>>>();
    
    rsx!{
        div {
            onmouseenter: move |_data| {
                state.set(Some(route.route_name()));
                notify_send(Some(tip), None);
            },
            onmouseleave: move |_data| notify_send(None, Some(0)),
            onclick: move |_data| notify_send(None, Some(1500)),
            Link { to: route.clone(),
                {icon}
                if route.english_name() == "error" {
                    del { "{route.chinese_name()}" }
                } else {
                    "{route.chinese_name()}"
                }
            }
        }
    }
}



use dioxus::prelude::*;
use crate::components::Profile;
#[allow(unused)]
use crate::utils::notify_send;
// use const_format::concatcp;

#[component]
pub fn Home() -> Element {
    // const _A: Asset = asset!("/assets/content/home.html");
    // let dir = include_dir!("$CARGO_MANIFEST_DIR/target/dx/ferlora/debug/web/public/assets");
    // let a = dir.get_file(_A.resolve().strip_prefix("/assets").to_str().unwrap()).unwrap();

    rsx! {
        Profile {}

        div { class: "min-h-screen",
            div { "=> 随意探索吧!! " "可能会有" span { class: "font-bold", " 惊喜 " } "哦~" }
            div { "这个博客本身就是一个 rust 的项目: " a { class: "hover:underline",  href: "https://github/jedsek/ferlory" } "https://github/jedsek/ferlory" }


            Details {
                summary: "我是谁?",
                li { "网名有 Jedsek, 柳下川, 清贫 等" }
                li { "爱好 ACGN(Animation <=> Comic <=> Game <=> Novel), 游戏偏好 2D, 如 SANABI(闪避刺客), Hollow-knights(空洞骑士)等" }
                li { "追寻艺术之人, 兴趣使然之人, 沉溺幻想之人, 拥抱孤独之人, 堕入虚无之人, 热爱世界之人" }
                li { "熟人面前放肆无比, 陌生人面前唯唯诺诺, 讨厌人挤人的环境(比如漫展), 是传统向宅男" }
                li { "东方Project中, 我推的角色是 依神紫菀(我头像就是这位哦, 她可爱吧?!)" }
                li { "即使感到迷茫也要前行, 我们都是......迷途之子啊! (?" }
            }

            Details {
                summary: "博客介绍",
                li { "这里是我的个人博客, 内容包括计算机方面的文章, 自己写的小说, 一些 galgame 中的语录, 乱七八糟的想法等" }
                li { "右侧是 目录(TOC) 哦, 页面内存在大标题时会自动生成, 支持 1 级与 2 级标题, 前往 编程 试试? 查看更多归纳好的文章" }
                li { "本站使用 dioxus 构建, 迁移历史为 hexo-next -> zola -> dioxus " del { "(做到了 pure rust/wasm && no javascript)" } }
                li { del { "手机/平板上有些地方可能不怎么适配 (因为我也前端恐惧症, 而且......我懒得改啦!!!)" } }
                li { "手机/平板上已经进行适配, 但为了最佳体验请用电脑查看喔!" }
            }

            Details {
                summary: "乱七八糟的想法与吐槽",
                li { "受不了了, css 好难, 我一顿乱糊结果居然 ok 了, 而且标准也很杂乱啊啊啊啊" }
                li { "麻麻的, 高中牲果然不是人, 大好的国庆, 大好的假期, 3天, 噫! 3天! 哈哈呵呵呵呵哈哈哈哈哈哈" }
                li { "本站还在飞速优化优化inging!!! 等我看完小说打会音游吃掉手里的串串就打开电脑敲代...等等, 新番更新日?! 受不鸟了" }
                li { "又是被 rust 生命周期暴打, 被编译器爸爸摁在地上锤的一天" }
            }
        }

        // div { class: "text-lg", oncopy: move |_data| notify_send(Some("呐, 文本已经复制好了"), Some(1000)),
        //     p { "大家好" }
        //     div { dangerous_inner_html: include_str!("../../assets/content/home.html") }
        // }
    }
}


#[component]
pub fn Details(summary:&'static str, children: Element) -> Element {
    let mut hidden = use_signal(|| true);
    
    rsx!{
        div {
            class: "mx-1 my-5",
            class: "transition-all duration-500 ease-in-out",
            div {
                class: "text-xl w-fit m-2",
                onmouseenter: move |_data| *hidden.write() = false,
                onclick: move |_data| *hidden.write() ^= true,
                i { class: "iconfont icon-arrow-down  text-xl!", class: if *hidden.read() { "hidden" } else { "inline" } }
                i { class: "iconfont icon-arrow-right text-xl!", class: if *hidden.read() { "inline" } else { "hidden" }}
                {summary}
            }
            ul {
                class: if *hidden.read() { "opacity-0 hidden" } else { "opacity-100 block" },
                class: "transition-all duration-500 ease-in-out",
                {children}
            }

        }
    }
}

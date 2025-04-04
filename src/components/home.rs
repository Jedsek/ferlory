use dioxus::prelude::*;
use crate::components::Profile;
use crate::utils::notify_send;
// use const_format::concatcp;

#[component]
pub fn Home() -> Element {
    const _A: Asset = asset!("/assets/content/home.html");
    // let dir = include_dir!("$CARGO_MANIFEST_DIR/target/dx/ferlora/debug/web/public/assets");
    // let a = dir.get_file(_A.resolve().strip_prefix("/assets").to_str().unwrap()).unwrap();

    rsx! {
        Profile {}
        div { class: "content text-[4vw] sm:text-[2rem]",
            "请随意进行探索吧! 可能会有 惊喜 哦~"
        }
        
        p { "我是谁?" }

        ul {
            li { "网名有 Jedsek, 柳下川, 清贫 等" }
            li { "爱好 ACG 文化, 是 galgame/轻小说/番剧/漫画 的狂热玩家, 喜欢 2d 游戏(如 SANABI(闪避刺客), Hollow-knights(空洞骑士)等)" }
            li { "追寻艺术之人, 兴趣使然之人, 沉溺幻想之人, 拥抱孤独之人, 堕入虚无之人, 热爱世界之人" }
            li { "熟悉的人面前放肆无比, 陌生的人面前唯唯诺诺, 讨厌人挤人的环境(比如漫展, 所以至今未去), 是传统意义上的宅男" }
            li { "东方Project里面我推的角色是 依神紫菀(我头像就是这位哦, 可爱吧可爱吧可爱吧?!)" }
            li { "即使感到迷茫也要前行, 我们都是......迷途之子啊! (?" }
        }

        p { "博客介绍" }

        ul {
            li { "这里是我的个人博客, 内容包括计算机方面的文章, 自己写点小说, 记录一些推 galgame 时语录, 还有乱七八糟的想法等" }
            li { "右侧是 目录(TOC) 哦, 页面内存在大标题时会自动生成, 支持 1 级与 2 级标题, 前往 编程 试试? 查看更多归纳好的文章" }
            li { "本站使用 zola 搭建, 从最初的 hexo-next 主题迁移而来, 前端废柴写了点垃圾 html/js/css" }
            li { "手机/平板上有些地方可能不怎么适配 (因为我也前端恐惧症, 而且......我懒得改啦!!!)" }
        }


        div { class: "text-lg", oncopy: move |_data| notify_send(Some("呐, 文本已经复制好了"), Some(1000)),
            p { "大家好" }
            div { dangerous_inner_html: include_str!("../../assets/content/home.html") }
        }
    }
}



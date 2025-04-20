use dioxus::prelude::*;
use ferlory_components::*;
use crate::Route;

#[component]
pub fn Programming() -> Element {
    rsx! {
        blockquote { "本页面记载一些计算机方面的技术文章" }
        
        Intro {}

        SplitLine {}
        
        H1 { text: "rust" }
        Todo { completed: true, Link { to: Route::Categories { name: "rust-tui".into() }, "tui" } }
        Todo { completed: true, Link { to: Route::SinglePost { name: "rust-common-errors".into() }, "rust中那些常见的错误" } }
        Todo { completed: true, Link { to: Route::SinglePost { name: "rust-clap-intro".into() }, "用 clap-rs 写一个命令行" } }
        Todo { completed: true, Link { to: Route::Categories { name: "rust-typed-magic".into() }, "rust中的类型魔法" } }
        Todo { completed: false, Link { to: Route::Categories { name: "rust-gui".into() }, "gui(gtk4/iced/dioxus)" } }
        Todo { completed: false, Link { to: Route::Categories { name: "rust-atomics-and-locks".into() }, "并发中的原子与锁" } }

        Details {
            summary: rsx! { "Other(todo)" },
            
            H1 { text: "ocaml" }

            H1 { text: "gleam" }

            H1 { text: "haskell" }

            H1 { text: "scala" }
        }

        H1 { id: "desktop-beauty", text: "桌面美化" }
        Todo { completed: true, Link { to: Route::Categories { name: "sicp-answers".into() }, "GNOME 入坑指南" } }


        H1 { text: "SICP" }
        "关于 SICP(计算机程序的构造与解释) 这本书的思考, 以及课后习题答案" br {}
        "暂时先做个目录, 或许是以年为单位进行更新, 有时间再说吧 QAQ" br {}
        br {}
        Todo { completed: true, Link { to: Route::Categories { name: "sicp-answers".into() }, "解题集" } }

        H1 { id: "text-editing", text: "文本编辑" }
        Todo { completed: true, Link { to: Route::Categories { name: "zed-blog-translation".into() }, "Zed's Blog(博客翻译)" } }


        // PostCard { serioes: "rust-tui", "rust-tui" }
        // TypstPost { path: "home" }
    }
}

#[component]
pub fn Intro() -> Element {
    rsx! {
        "其实我是一只会咕咕叫的肥美鸽子, 虽然写博客好麻烦, 但我还是尽量保持更新吧? 只要有空? " del { "(应该吧)" } " :)" br {}
        "请根据需要选择文章, 然后进行传送吧! "  del { "(记好了, 传送魔法我只教你这么一次哦!)" } br {}
        br {}
        "有一些文章难度比较大, 资料也缺乏, 整理起来费时费力, 鉴于个人能力正在缓慢学习ing" br {}
        "以下大部分文章还是我高中时期写的, 像是 haskell 之类的系列理所当然地被鸽掉了! !" br {}
        br {}
        br {}
        br {}
        "关于每行前面框框的说明:" br {}
        br {}
        Todo { completed: true,  "<--这个表示以下情况之一: " span { class: "text-pink-400", "写完了" } " or " span { class: "text-sky-400", "该系列文章间关联不大随缘更新" } }
        Todo { completed: false, "<--这个表示还没更完 " del { "(新建文件夹! 咕咕咕咕咕!!!)" } } br {}
    }
}

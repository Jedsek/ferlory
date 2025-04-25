use dioxus::prelude::*;
use ferlory_components::*;
use crate::utils::link_gen;

#[component]
pub fn Category() -> Element {
    let mut links = link_gen::new_series("rust-tui");
    
    rsx! {
        div { class: "flex justify-center text-3xl my-6 opacity-80", "rust-tui"  }
        
        "本系列, 我们将会了解如何使用 ratatui-rs(rust里大名鼎鼎的tui库) 来编写一个 tui 程序" br{}
        "比如你听说过的 vim, ranger, gitui, bottom 等非常有用的终端工具, 都属于 tui 的范畴" br{}
        br{}
        "p2 涉及了 ratatui 中模板项目的架构" br{}
        "p3 则讲解了一些常见的概念, 比如 tui 中管道/重定向的作用与注意点" br{}
        br{}
        "之后可能会多来点实战的小项目的, 不过我真的挺懒的"
        "嘿嘿, 你想啊, 5 个月后更一章(p3), 那章还是某天突然想写了, 然后一天内写完的, 爷真随性 (x" br{}
        br{}

        Todo { completed: true,  {links.next("基础架构")} }
        Todo { completed: true,  {links.next("终端中的基础概念")} }
    }
}

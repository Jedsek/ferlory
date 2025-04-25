use crate::Route;
use dioxus::prelude::*;
use ferlory_components::{Footer, Notification};
use ferlory_utils::notify_send;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "relative",
            div {
                class: "border-b-2 w-full border-slate-700 bg-ink-purple z-10 fixed top-0",
                div {
                    class: "flex flex-row w-fit mx-auto *:m-1",
                    class: "sm:w-auto sm:*:mx-2",
                    NavItem { route: Route::Home {}, name: "首页" }
                    NavItem { route: Route::Programming {}, name: "编程" }
                    NavItem { route: Route::About {}, name: "关于" }
                    NavItem { route: Route::Moments {}, name: "时刻" }
                    NavItem { route: Route::Friends {}, name: "友链" }
                    NavItem { route: Route::Fantasy {}, name: "幻想" }
                    NavItem { route: Route::ErrorPage {}, name: "错误" }
                }
            }
            div {
                class: "min-h-screen pt-10 bg-ink-purple px-2 sm:px-3",
                oncopy: move |_data| notify_send(Some("喂, 文本已经复制好了"), Some(1500)),
                Outlet::<Route> {}
                Footer {}
            }
            Notification { }
        }
    }
}

#[component]
pub fn NavItem(route: Route, name: &'static str) -> Element {
    let current_route = use_route::<Route>();
    let is_selected = match (&route, &current_route) {
        (r1, r2) if r1 == r2 => true, // 判断 route 和 current_route 相等
        (Route::Programming {}, Route::Categories {..})
        | (Route::Programming {}, Route::SeriesPost {..})
        | (Route::Programming {}, Route::SinglePost {..}) => true,
        _ => false,
    };

    rsx! {
        Link {
            class: if is_selected { "no-underline! text-fuchsia-400" } else { "no-underline!" },
            to: route.clone(),
            if is_selected {
                i { class: "iconfont icon-arrow-right-thin" }
            }
            {name}
        }
    }
}

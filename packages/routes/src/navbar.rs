use dioxus::prelude::*;
use ferlory_components::{Footer, Notification};
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "relative",
            div {
                class: "border-b-2 w-full border-slate-700 bg-ink-purple z-10 fixed top-0",
                div {
                    class: "flex flex-row w-fit mx-auto *:m-1",
                    class: "",
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
            div { class: "pt-10 bg-ink-purple px-2",
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
    
    rsx! {
        Link {
            class: if current_route == route { "no-underline! text-fuchsia-400" } else { "no-underline!" },
            to: route.clone(),
            if current_route == route {
                i { class: "iconfont icon-arrow-right-thin" }
            }
            {name}
        }
    }
}

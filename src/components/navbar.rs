use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            div {
                class: "flex flex-row border-b-1",
                class: "*:m-1 sm:*:m-2 **:no-underline! **:hover:underline!",
                div { Link { to: Route::Home {}, "首页" } },
                div { Link { to: Route::About {}, "关于" } },
                div { Link { to: Route::Friends {}, "友链" } },
                div { Link { to: Route::Programming {}, "编程" } },
                div { Link { to: Route::Fantasy {}, "幻想" } },
                div { Link { to: Route::ErrorPage {}, "错误" } },
            }
        }
        Outlet::<Route> {}
    }
}

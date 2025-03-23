use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        nav {
            ul { class: "flex flex-row *:m-1 *:hover:underline",
                li { Link { to: Route::Home {}, "首页" } },
                li { Link { to: Route::Programming {}, "编程" } },
                li { Link { to: Route::Fantasy {}, "幻想" } },
                li { Link { to: Route::ErrorPage {}, "错误" } },
            }
        }
        Outlet::<Route> {}
    }
}

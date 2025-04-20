use crate::{components::Notification, Route};
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut _state: Signal<Option<&'static str>> = use_signal(|| None);
    use_context_provider(|| _state);

    rsx! {
        nav {
            class: "border-b-2 border-slate-700",
            div {
                class: "flex flex-row w-fit mx-auto *:m-1 *:no-underline! *:hover:underline!",
                class: "*:hover:scale-110 *:hover:font-semibold",
                class: "sm:w-auto sm:*:mx-2",
                Link { to: Route::Home {}, "首页" }
                Link { to: Route::Programming {}, "编程" }
                Link { to: Route::About {}, "关于" }
                Link { to: Route::Moments {}, "时刻" }
                Link { to: Route::Friends {}, "友链" }
                Link { to: Route::Fantasy {}, "幻想" }
                Link { to: Route::ErrorPage {}, "错误" }
            }
        }
        Outlet::<Route> {}
        Notification { }
    }
}

// #[component]
// pub fn NavItem(route: &'static str, tip_text: &'static str, children: Element) -> Element {
//     let mut state = use_context::<Signal<Option<&'static str>>>();
    
//     rsx!{
//         div {
//             onmouseenter: move |_data| {
//                 state.set(Some(tip_route));
//                 notify_send(Some(tip_text), None);
//             },
//             onmouseleave: move |_data| {
//                 notify_send(None, Some(0));
//             },
//             onclick: move |_data| {
//                 notify_send(None, Some(1500));
//             },
//             {children}
//         }
//     }
// }

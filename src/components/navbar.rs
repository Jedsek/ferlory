use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut _state: Signal<Option<&'static str>> = use_signal(|| None);
    use_context_provider(|| _state);

    rsx! {
        nav {
            class: "flex flex-row border-b-1",
            class: "*:m-1 sm:*:m-2 **:no-underline! **:hover:underline!",
            div { Link { to: Route::Home {}, "首页" } },
            div { Link { to: Route::About {}, "关于" } },
            div { Link { to: Route::Friends {}, "友链" } },
            div { Link { to: Route::Programming {}, "编程" } },
            div { Link { to: Route::Fantasy {}, "幻想" } },
            div { Link { to: Route::ErrorPage {}, "错误" } },
        }
        Outlet::<Route> {}
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

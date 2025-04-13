mod components;
mod route;
mod utils;

use dioxus::prelude::*;
use components::Notification;
use route::Route;
use utils::notify_send;

fn main() {
    utils::launch_app(|| {
        rsx! {
            document::Title { "柳下川 && Ferlory" }
            document::Link { rel: "shortcut icon", href: asset!("/assets/images/avatar.avif"), type: "image/x-icon" }
            document::Stylesheet { href: asset!("/assets/styles/tailwind.css") }
            document::Stylesheet { href: asset!("/assets/iconfonts/iconfont.css") }
            document::Stylesheet { href: asset!("/assets/styles/typst.css") }
            document::Stylesheet { href: "https://chinese-fonts-cdn.deno.dev/packages/maple-mono-cn/dist/MapleMono-CN-Regular/result.css" }
            document::Stylesheet { href: "https://chinese-fonts-cdn.deno.dev/packages/maple-mono-cn/dist/MapleMono-CN-Italic/result.css" }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-Regular/result.css") }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-Italic/result.css") }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-Bold/result.css") }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-SemiBold/result.css") }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-BoldItalic/result.css") }
            // document::Stylesheet { href: asset!("/assets/fonts/MapleMono-NF-CN-Medium/result.css") }
            Notification { }
            div {
                class: "min-h-screen mx-1 sm:mx-2",
                oncopy: move |_data| notify_send(Some("喂, 文本已经复制好了"), Some(1500)),
                Router<Route> {}
            }
        }
    })
}


#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

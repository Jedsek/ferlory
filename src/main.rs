mod components;
mod route;
mod utils;

use dioxus::prelude::*;
use components::Notification;
use route::Route;

const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind.css");
const ICONFONT_CSS: Asset = asset!("/assets/iconfonts/iconfont.css");
const TYPST_CSS: Asset = asset!("/assets/styles/typst.css");

fn main() {
    utils::launch_app(|| {
        rsx! {
            document::Title { "柳下川的博客" }
            document::Link { rel: "shortcut icon", href: asset!("/assets/images/avatar.avif"), type: "image/x-icon" }
            document::Stylesheet { href: "{TAILWIND_CSS}" }
            document::Stylesheet { href: "{ICONFONT_CSS}" }
            document::Stylesheet { href: "{TYPST_CSS}" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Regular/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Italic/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Bold/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-SemiBold/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-BoldItalic/result.css" }
            Notification { }
            Router<Route> {}
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

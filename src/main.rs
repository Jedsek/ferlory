mod components;
mod route;
mod utils;

use dioxus::prelude::*;
use components::Notification;
use route::Route;

fn main() {
    utils::launch_app(|| {
        rsx! {
            document::Title { "柳下川 && Ferlory" }
            document::Link { rel: "shortcut icon", href: asset!("/assets/images/avatar.avif"), type: "image/x-icon" }
            document::Stylesheet { href: "/assets/styles/tailwind.css" }
            document::Stylesheet { href: "/assets/iconfonts/iconfont.css" }
            document::Stylesheet { href: "/assets/styles/typst.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Regular/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Italic/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-Bold/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-SemiBold/result.css" }
            document::Stylesheet { href: "/assets/fonts/MapleMono-NF-CN-BoldItalic/result.css" }
            Router<Route> {}
            Notification { }
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

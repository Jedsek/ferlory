mod components;
mod route;
mod utils;

use dioxus::prelude::*;
use route::Route;

const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Regular.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Bold.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-BoldItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-ExtraBold.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-ExtraBoldItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-ExtraLight.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-ExtraLightItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Italic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Light.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-LightItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Medium.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-MediumItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Regular.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-SemiBold.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-SemiBoldItalic.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-Thin.woff2");
const _: Asset = asset!("/assets/fonts/MapleMono-Woff2/MapleMono-ThinItalic.woff2");

const STYLE_CSS: Asset = asset!("/assets/styles/fonts.css", CssAssetOptions::new().with_preload(true));
const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind.css", CssAssetOptions::new().with_preload(true));
const TYPST_CSS: Asset = asset!("/assets/styles/typst.css", CssAssetOptions::new().with_preload(true));


fn main() {
    utils::launch_app(|| {
        rsx! {
            document::Stylesheet { href: "{TAILWIND_CSS}" }
            document::Stylesheet { href: "{STYLE_CSS}" }
            document::Stylesheet { href: "{TYPST_CSS}" }
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

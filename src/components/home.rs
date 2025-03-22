use crate::components::Avatar;
use dioxus::prelude::*;
// use const_format::concatcp;

#[component]
pub fn Home() -> Element {
    const _A: Asset = asset!("/assets/content/home.html");
    // let dir = include_dir!("$CARGO_MANIFEST_DIR/target/dx/ferlora/debug/web/public/assets");
    // let a = dir.get_file(_A.resolve().strip_prefix("/assets").to_str().unwrap()).unwrap();

    rsx! {
        Avatar {}
        div { dangerous_inner_html: include_str!("../../assets/content/home.html") }
    }
}

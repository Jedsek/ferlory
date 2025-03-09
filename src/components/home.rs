use crate::components::Avatar;
// use const_format::concatcp;

use dioxus::prelude::*;
use include_dir::{include_dir, Dir};

#[component]
pub fn Home() -> Element {
    const _A: Asset = asset!("/assets/content/home.html");
    // let dir = include_dir!("$CARGO_MANIFEST_DIR/target/dx/ferlora/debug/web/public/assets");
    // let a = dir.get_file(_A.resolve().strip_prefix("/assets").to_str().unwrap()).unwrap();

    rsx! {
        Avatar {},
    }
}

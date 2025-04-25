use dioxus::prelude::*;
use ferlory_components::TypstPost;

#[component]
pub fn SeriesPost(series: String, name: String) -> Element {
    let path = series.trim_end_matches('/').to_string() + "/" + &name;
    let path = path.as_str();

    rsx! {
        TypstPost { path }
    }
}

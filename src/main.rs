mod components;
mod route;
mod utils;

use dioxus::prelude::*;
use route::Route;

fn main() {
    utils::launch_app(|| {
        rsx! {
            document::Stylesheet {
                href: asset!("/assets/tailwind.css")
            }
            document::Stylesheet {
                href: asset!("/assets/style.css")
            }
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

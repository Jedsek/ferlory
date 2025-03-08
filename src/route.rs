use crate::components::{ErrorPage, Fantasy, Home, Other, Programming, Sidebar};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Sidebar)]
        #[route("/")]
        Home {},
        #[route("/categories")]
        Programming {},
        #[route("/fantasy")]
        Fantasy {},
        #[route("/404")]
        ErrorPage {},
    #[end_layout]

    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/other")]
    Other {}
}

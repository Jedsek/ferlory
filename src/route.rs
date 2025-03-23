use crate::components::{About, ErrorPage, Fantasy, Friends, Home, Other, Programming, Sidebar};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[layout(Sidebar)]
        #[route("/programming")]
        Programming {},
        #[route("/fantasy")]
        Fantasy {},
        #[route("/friends")]
        Friends {},
        #[route("/about")]
        About {},
        #[route("/404")]
        ErrorPage {},
    #[end_layout]

    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/other")]
    Other {}
}

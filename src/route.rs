use crate::components::{About, ErrorPage, Fantasy, Friends, Home, Other, Moments, Programming, NavBar};
use dioxus::prelude::*;

#[derive(Debug, Copy, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[layout(NavBar)]
        #[route("/moments")]
        Moments {},
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

impl Route {
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Self::Home {} => "首页",
            Self::Moments {} => "时刻",
            Self::Programming {} => "编程",
            Self::Fantasy {} => "幻想",
            Self::Friends {} => "友链",
            Self::About {} => "关于",
            Self::ErrorPage {} => "错误",
            Self::Other {} => "其他",
        }
    }

    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Home {} => "home",
            Self::Moments {} => "moments",
            Self::Programming {} => "programming",
            Self::Fantasy {} => "fantasy",
            Self::Friends {} => "friends",
            Self::About {} => "about",
            Self::ErrorPage {} => "error",
            Self::Other {} => "other",
        }
    }

    pub fn route_name(&self) -> &'static str {
        match self {
            Self::Home {} => "/home",
            Self::Moments {} => "/moments",
            Self::Programming {} => "/programming",
            Self::Fantasy {} => "/fantasy",
            Self::Friends {} => "/friends",
            Self::About {} => "/about",
            Self::ErrorPage {} => "/error",
            Self::Other {} => "/other",
        }
    }
}

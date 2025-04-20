#![allow(unreachable_patterns)]

use crate::components::routes::*;
use crate::components::NavBar;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/moments/")]
        Moments {},
        #[route("/programming/")]
        Programming {},
        #[route("/fantasy/")]
        Fantasy {},
        #[route("/friends/")]
        Friends {},
        #[route("/about/")]
        About {},
    #[end_layout]

    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/404/")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::ErrorPage {})]
    ErrorPage {},
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
            _ => unreachable!()
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
            _ => unreachable!()
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
            _ => unreachable!()
        }
    }
}

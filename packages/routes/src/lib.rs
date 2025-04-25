#![allow(unreachable_patterns, non_snake_case)]

use dioxus::prelude::*;

pub mod utils;

mod navbar;
pub use navbar::NavBar;

mod home;
pub use home::Home;

mod moments;
pub use moments::Moments;

mod programming;
pub use programming::Programming;

mod fantasy;
pub use fantasy::Fantasy;

mod friends;
pub use friends::Friends;

mod about;
pub use about::About;

mod error_page;
pub use error_page::ErrorPage;

mod categories;
pub use categories::Categories;

mod series_post;
pub use series_post::SeriesPost;

mod single_post;
pub use single_post::SinglePost;

mod other;
pub use other::Other;

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

        #[route("/categories/:name")]
        Categories { name: String },

        #[nest("/posts")]
            #[route("/:series/:name")]
            SeriesPost { series: String, name: String },

            #[route("/:name")]
            SinglePost { name: String },
        #[end_nest]

        #[route("/404/")]
        ErrorPage {},
    #[end_layout]

    
    #[route("/other")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::ErrorPage {})]
    Other { segments: Vec<String> },

    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
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
            _ => unreachable!(),
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
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }
}

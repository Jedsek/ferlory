use dioxus::prelude::*;
use crate::Route;

pub struct LinkSeries {
    series: &'static str,
    index: usize,
}

pub fn new_series(series: &'static str) -> LinkSeries {
    LinkSeries { series, index: 0 }
}

impl LinkSeries {
    pub fn next(&mut self, text: &'static str) -> Element {
        self.index += 1;

        let route = Route::SeriesPost { series: self.series.to_string(), name: format!("p{}", self.index) };

        rsx! {
            Link { to: route, {text} }
        }
    }
}

pub fn next_single(href: &'static str, text: &'static str) -> Element {
    let route = Route::SinglePost { name: {href.to_string()} };
    rsx! {
        Link { to: route, {text} }
    }
}

pub fn next_category(href: &'static str, text: &'static str) -> Element {
    let route = Route::Categories { name: {href.to_string()} };
    rsx! {
        Link { to: route, {text} }
    }
}

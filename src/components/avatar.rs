use dioxus::{dioxus_core::VPlaceholder, logger::tracing, prelude::*};
use gloo::timers::callback::Interval;
use rand::random_range;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical
}

#[component]
pub fn Avatar() -> Element {
    const ASSET: Asset = asset!("/assets/images/avatar.avif");

    let width = ["w-1", "w-2", "w-3", "w-4", "w-5"];
    let mut count = use_signal(|| 0);
    use_future(move || async move {
        Interval::new(200, move || {

            count.set(random_range(0..5));
            // tracing::debug!("{}", count);
        }).forget();
    });
    rsx! {
        div { class: "flex w-50 m-10",
            img { src: "{ASSET}"  }
            AvatarBar { direction: Direction::Horizontal }
        }

    }
}

#[component]
pub fn AvatarBar(direction: Direction, ) -> Element {
    let mut count = use_signal(|| 0);
    use_future(move || async move {
        Interval::new(50, move || {

            count.set(random_range(0..5));
            // tracing::debug!("{}", count);
        }).forget();
    });

    rsx! {
        div { class: "flex flex-col",
            for a in (1..=count()) {
                div { class: "w-2 h-4 bg-[violet] m-[1px]", Placeholder { } }
            }
        }
    }  
}



#[component]
pub fn Placeholder() -> Element {
    rsx! {
        div { class: "opacity-0", "1" }
    }
  
}

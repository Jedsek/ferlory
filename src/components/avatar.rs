use dioxus::prelude::*;
use gloo::timers::callback::Interval;
use rand::random_range;
use crate::components::Placeholder;


#[component]
pub fn Avatar() -> Element {
    const AVATAR: Asset = asset!("/assets/images/avatar.avif");
    const AVATAR_BORDER: Asset = asset!("/assets/images/avatar_border.avif");

    rsx! {
        img { class: "w-40", src: "{ANI1}"  }
        img { class: "w-40", src: "{ANI2}"  }
        div { class: "flex flex-row justify-center sm:justify-start",
            div { class: "flex flex-col",
                Corner { }
                div { class: "grow",
                    Bar { direction: Direction::LeftToRight, len: 20 }
                }
                Corner { }
            }
            div { class: "flex flex-col",
                div { class: "grow",
                    Bar { direction: Direction::BottomToTop, len: 20 }
                }
                img { class: "w-40", src: "{AVATAR}"  }
                div { class: "grow",
                    Bar { direction: Direction::BottomToTop, len: 20 }
                }
            }
            div { class: "flex flex-col",
                Corner { }
                div { class: "grow",
                    Bar { direction: Direction::LeftToRight, len: 20 }
                }
                Corner { }
            }
        }
    }
}

#[component]
pub fn Bar(direction: Direction, len: usize) -> Element {
    let cell_size = match direction {
        Direction::BottomToTop | Direction::TopToBottom => "w-4 h-2",
        _ => "w-2 h-4"
    };

    let cell_spacing = match direction {
        Direction::BottomToTop | Direction::TopToBottom => "py-1",
        _ => "px-1 py-1"
    };

    let flex_direction = match direction {
        Direction::BottomToTop | Direction::TopToBottom => "flex flex-row",
        _ => "flex flex-col "
    };
    
    let mut count = use_signal(|| 0);
    use_future(move || async move {
        Interval::new(200, move || {

            count.set(random_range(0..5));
            // tracing::debug!("{}", count);
        }).forget();
    });

    rsx! {
        div { class: "{flex_direction}",
            for _a in (0..=count()) {
                div { class: "{cell_size} {cell_spacing} bg-purple-500" }
            }
        }
    }  
}

#[component]
pub fn Corner() -> Element {
    rsx! {
        div { class: "w-4 h-4 bg-linear-to-bl from-violet-500 to-fuchsia-500 animate-pulse",
            Placeholder { }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Avatar() -> Element {
    const ASSET: Asset = asset!("/assets/images/avatar.avif");
    rsx! {
        div { class: "flex w-[20rem] m-10",
            img { src: "{ASSET}"  }

        }

    }
}

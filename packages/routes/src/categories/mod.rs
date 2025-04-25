use dioxus::prelude::*;

macro_rules! categories {
    ($($category:ident)*) => {
        $(
            pub mod $category;
        )*
    };
}

categories!(
    rust_tui
    rust_typed_magic
    rust_gui
);

#[component]
pub fn Categories(name: String) -> Element {
    ferlory_macros::goto_category!(name.as_str(),
        rust_tui,
        rust_typed_magic,
        rust_gui
    )
}

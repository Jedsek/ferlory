#![allow(non_snake_case)]

mod placeholder;
#[allow(unused)]
pub use placeholder::Placeholder;

mod notify;
pub use notify::Notification;
pub use notify::STATE;

mod footer;
pub use footer::Footer;

mod anchor;
pub use anchor::{H1, H2};

mod details;
pub use details::Details;

mod split_line;
pub use split_line::SplitLine;

mod post;
pub use post::TypstPost;

mod todo;
pub use todo::Todo;

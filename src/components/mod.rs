// #![allow(unused)]

pub mod post_components;
pub mod routes;

mod placeholder;
#[allow(unused)]
pub use placeholder::Placeholder;

mod navbar;
pub use navbar::NavBar;

mod notify;
pub use notify::Notification;
pub use notify::STATE;

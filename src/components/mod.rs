// #![allow(unused)]

mod posts;

mod placeholder;
#[allow(unused)]
pub use placeholder::Placeholder;

mod profile;
pub use profile::Profile;

mod sidebar;
pub use sidebar::Sidebar;

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

mod page_not_found;
pub use page_not_found::Other;

mod notify;
pub use notify::Notification;
pub use notify::STATE;

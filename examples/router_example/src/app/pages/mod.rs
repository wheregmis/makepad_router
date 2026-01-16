pub mod detail;
pub mod home;
pub mod not_found;
pub mod settings;

pub use detail::DetailController;
pub use home::HomeController;
pub use not_found::NotFoundController;
pub use settings::SettingsController;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    home::live_design(cx);
    settings::live_design(cx);
    detail::live_design(cx);
    not_found::live_design(cx);
}

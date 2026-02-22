pub mod detail;
pub mod home;
pub mod not_found;
pub mod settings;

pub use detail::DetailController;
pub use home::HomeController;
pub use not_found::NotFoundController;
pub use settings::SettingsController;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    home::script_mod(vm);
    settings::script_mod(vm);
    detail::script_mod(vm);
    not_found::script_mod(vm);
}

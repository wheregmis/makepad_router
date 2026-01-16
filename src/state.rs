use crate::navigation::NavigationHistory;
use makepad_micro_serde::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct RouterState {
    pub history: NavigationHistory,
    pub url_path_override: Option<String>,
}

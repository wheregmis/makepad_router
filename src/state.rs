use crate::navigation::NavigationHistory;
use makepad_micro_serde::*;

/// Serializable router state (history + optional URL override).
#[derive(Clone, Debug, Default, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct RouterState {
    /// Navigation history stack.
    pub history: NavigationHistory,
    /// Optional URL override used by the widget on web targets.
    pub url_path_override: Option<String>,
}

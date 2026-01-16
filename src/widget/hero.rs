use crate::hero::HeroPair;

#[derive(Clone, Debug)]
pub(super) struct HeroTransitionState {
    pub(super) capture_done: bool,
    pub(super) pairs: Vec<HeroPair>,
}

impl Default for HeroTransitionState {
    fn default() -> Self {
        Self {
            capture_done: false,
            pairs: Vec::new(),
        }
    }
}


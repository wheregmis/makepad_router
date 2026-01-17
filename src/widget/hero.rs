use crate::hero::HeroPair;

#[derive(Clone, Debug, Default)]
pub(super) struct HeroTransitionState {
    pub(super) capture_done: bool,
    pub(super) pairs: Vec<HeroPair>,
}


use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct SharedState {
    pub auth_logged_in: Arc<AtomicBool>,
    pub settings_dirty: Arc<AtomicBool>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            auth_logged_in: Arc::new(AtomicBool::new(false)),
            settings_dirty: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl SharedState {
    pub fn is_logged_in(&self) -> bool {
        self.auth_logged_in.load(Ordering::SeqCst)
    }

    pub fn set_logged_in(&self, v: bool) {
        self.auth_logged_in.store(v, Ordering::SeqCst);
    }

    pub fn is_dirty(&self) -> bool {
        self.settings_dirty.load(Ordering::SeqCst)
    }

    pub fn set_dirty(&self, v: bool) {
        self.settings_dirty.store(v, Ordering::SeqCst);
    }
}


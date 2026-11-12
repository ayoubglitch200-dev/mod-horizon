use std::sync::atomic::{AtomicBool, Ordering};

static MENU_OPEN: AtomicBool = AtomicBool::new(false);

pub fn toggle_menu() {
    MENU_OPEN.fetch_xor(true, Ordering::Relaxed);
}

pub fn menu_is_open() -> bool {
    MENU_OPEN.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_starts_closed_and_toggles() {
        MENU_OPEN.store(false, Ordering::Relaxed);
        assert!(!menu_is_open());
        toggle_menu();
        assert!(menu_is_open());
        toggle_menu();
        assert!(!menu_is_open());
    }
}

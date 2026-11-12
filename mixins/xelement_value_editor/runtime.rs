use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MENU_OPEN: AtomicBool = AtomicBool::new(false);
static SELECTED_INDEX: AtomicUsize = AtomicUsize::new(0);
const ITEM_COUNT: usize = 4;

pub fn toggle_menu() {
    let was_open = MENU_OPEN.fetch_xor(true, Ordering::Relaxed);
    if !was_open {
        SELECTED_INDEX.store(0, Ordering::Relaxed);
    }
}

pub fn close_menu() {
    MENU_OPEN.store(false, Ordering::Relaxed);
}

pub fn menu_is_open() -> bool {
    MENU_OPEN.load(Ordering::Relaxed)
}

pub fn selected_index() -> usize {
    SELECTED_INDEX.load(Ordering::Relaxed)
}

pub fn select_next() {
    let _ = SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |index| {
        Some((index + 1) % ITEM_COUNT)
    });
}

pub fn select_previous() {
    let _ = SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |index| {
        Some((index + ITEM_COUNT - 1) % ITEM_COUNT)
    });
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

    #[test]
    fn selection_wraps_in_both_directions() {
        SELECTED_INDEX.store(0, Ordering::Relaxed);
        select_previous();
        assert_eq!(selected_index(), ITEM_COUNT - 1);
        select_next();
        assert_eq!(selected_index(), 0);
    }
}

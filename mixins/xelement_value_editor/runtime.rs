use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MENU_OPEN: AtomicBool = AtomicBool::new(false);
static SELECTED_INDEX: AtomicUsize = AtomicUsize::new(0);
static CURRENT_PAGE: AtomicUsize = AtomicUsize::new(0);
static ITEM_INDEX: AtomicUsize = AtomicUsize::new(0);
const CATEGORY_COUNT: usize = 4;

pub fn toggle_menu() {
    let was_open = MENU_OPEN.fetch_xor(true, Ordering::Relaxed);
    if !was_open {
        CURRENT_PAGE.store(0, Ordering::Relaxed);
        SELECTED_INDEX.store(0, Ordering::Relaxed);
        ITEM_INDEX.store(0, Ordering::Relaxed);
    }
}
pub fn close_menu() { MENU_OPEN.store(false, Ordering::Relaxed); }
pub fn menu_is_open() -> bool { MENU_OPEN.load(Ordering::Relaxed) }
pub fn selected_index() -> usize { SELECTED_INDEX.load(Ordering::Relaxed).min(CATEGORY_COUNT - 1) }
pub fn current_page() -> usize { CURRENT_PAGE.load(Ordering::Relaxed) }
pub fn item_index() -> usize { ITEM_INDEX.load(Ordering::Relaxed) }

pub fn select_next() {
    if current_page() == 0 {
        let _ = SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + 1) % CATEGORY_COUNT));
    } else {
        let count = match current_page() { 1 => 3, 2 => 1, _ => 1 };
        let _ = ITEM_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + 1) % count));
    }
}
pub fn select_previous() {
    if current_page() == 0 {
        let _ = SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + CATEGORY_COUNT - 1) % CATEGORY_COUNT));
    } else {
        let count = match current_page() { 1 => 3, 2 => 1, _ => 1 };
        let _ = ITEM_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + count - 1) % count));
    }
}
pub fn select_index(index: usize) {
    if current_page() == 0 && index < CATEGORY_COUNT { SELECTED_INDEX.store(index, Ordering::Relaxed); }
}
pub fn enter_category() {
    if current_page() == 0 {
        CURRENT_PAGE.store(selected_index() + 1, Ordering::Relaxed);
        ITEM_INDEX.store(0, Ordering::Relaxed);
    }
}
pub fn back_to_categories() {
    let page = current_page();
    if page != 0 {
        CURRENT_PAGE.store(0, Ordering::Relaxed);
        SELECTED_INDEX.store(page.saturating_sub(1).min(3), Ordering::Relaxed);
        ITEM_INDEX.store(0, Ordering::Relaxed);
    }
}

/// Change a supported local-player value. Returns false when that value is not
/// implemented or the local player is unavailable.
pub fn adjust_value(game: &mut crate::game::Game<'_>, direction: f32) -> bool {
    match current_page() {
        1 => {
            let Some(player) = game.player.as_mut() else { return false; };
            match item_index() {
                0 => {
                    let value = (player.health.health() + direction).max(0.0);
                    player.set_health(value, false).is_ok()
                }
                1 => {
                    player.health.food.food = (player.health.food.food + direction as i32).clamp(0, 20);
                    true
                }
                2 => {
                    let food = player.health.food.food as f32;
                    player.health.food.saturation = (player.health.food.saturation + direction).clamp(0.0, food);
                    true
                }
                _ => false,
            }
        }
        2 => {
            use horizon_world::attribute_map::ValueKind;
            let Some(player) = game.player.as_mut() else { return false; };
            let Some(key) = player.attributes.key("minecraft:movement_speed") else { return false; };
            let Ok(base) = player.attributes.value(Some(key.clone()), ValueKind::Base) else { return false; };
            let Some(handle) = player.attributes.get_instance(Some(key.clone())).ok().flatten() else { return false; };
            let new_base = (base + f64::from(direction) * 0.01).clamp(0.001, 1.0);
            if player.attributes.edit(&handle, |value, observer| value.set_base_value(new_base, observer)).is_err() {
                return false;
            }
            let Ok(computed) = player.attributes.value(Some(key), ValueKind::Computed) else { return false; };
            player.speed = computed as f32;
            true
        }
        _ => false,
    }
}

pub fn player_values(game: &crate::game::Game<'_>) -> [String; 3] {
    match game.player.as_ref() {
        Some(p) => [
            format!("{:.1}", p.health.health()),
            format!("{}", p.health.food.food),
            format!("{:.1}", p.health.food.saturation),
        ],
        None => ["N/A".into(), "N/A".into(), "N/A".into()],
    }
}
pub fn movement_value(game: &crate::game::Game<'_>) -> String {
    use horizon_world::attribute_map::ValueKind;
    let Some(player) = game.player.as_ref() else { return "N/A".into(); };
    let Some(key) = player.attributes.key("minecraft:movement_speed") else { return "N/A".into(); };
    player.attributes.value(Some(key), ValueKind::Computed)
        .map(|value| format!("{:.4}", value))
        .unwrap_or_else(|_| "N/A".into())
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
        CURRENT_PAGE.store(0, Ordering::Relaxed);
        SELECTED_INDEX.store(0, Ordering::Relaxed);
        select_previous();
        assert_eq!(selected_index(), CATEGORY_COUNT - 1);
        select_next();
        assert_eq!(selected_index(), 0);
    }
}

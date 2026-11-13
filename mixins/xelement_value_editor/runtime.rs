use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MENU_OPEN: AtomicBool = AtomicBool::new(false);
static SELECTED_INDEX: AtomicUsize = AtomicUsize::new(0);
static CURRENT_PAGE: AtomicUsize = AtomicUsize::new(0);
static ITEM_INDEX: AtomicUsize = AtomicUsize::new(0);
const CATEGORY_COUNT: usize = 4;
const PLAYER_ITEM_COUNT: usize = 9;
const EFFECT_IDS: [&str; 5] = ["minecraft:speed", "minecraft:strength", "minecraft:jump_boost", "minecraft:regeneration", "minecraft:invisibility"];

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
        let count = page_item_count(current_page());
        let _ = ITEM_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + 1) % count));
    }
}
pub fn select_previous() {
    if current_page() == 0 {
        let _ = SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| Some((i + CATEGORY_COUNT - 1) % CATEGORY_COUNT));
    } else {
        let count = page_item_count(current_page());
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


fn page_item_count(page: usize) -> usize {
    match page {
        1 => PLAYER_ITEM_COUNT,
        2 => 1,
        3 => 2,
        4 => EFFECT_IDS.len(),
        _ => 1,
    }
}

fn change_attribute(game: &mut crate::game::Game<'_>, id: &str, direction: f64, min: f64, max: f64) -> bool {
    use horizon_world::attribute_map::ValueKind;
    let Some(player) = game.player.as_mut() else { return false; };
    let Some(key) = player.attributes.key(id) else { return false; };
    let Ok(base) = player.attributes.value(Some(key.clone()), ValueKind::Base) else { return false; };
    let Ok(Some(handle)) = player.attributes.get_instance(Some(key.clone())) else { return false; };
    let next = (base + direction).clamp(min, max);
    if player.attributes.edit(&handle, |value, observer| value.set_base_value(next, observer)).is_err() {
        return false;
    }
    if id == "minecraft:max_health" { let _ = player.clamp_health_to_max(); }
    if id == "minecraft:movement_speed" { player.speed = next as f32; }
    true
}

fn effect_is_active(game: &crate::game::Game<'_>, index: usize) -> bool {
    let Some(id) = EFFECT_IDS.get(index) else { return false; };
    let short = id.strip_prefix("minecraft:").unwrap_or(id);
    game.potions.effects.get(short).is_some()
}

fn toggle_effect(game: &mut crate::game::Game<'_>, index: usize) -> bool {
    let Some(id) = EFFECT_IDS.get(index) else { return false; };
    let Some(def) = horizon_client::brewing::effect(id) else { return false; };
    if effect_is_active(game, index) {
        let effect_id = def.id;
        game.potions.effects.list.retain(|e| e.id != effect_id);
        game.apply_effect_status();
        true
    } else {
        let mut effect = horizon_client::effects::EffectInstance::new(def.id, 20 * 60 * 5, 0);
        effect.visible = true;
        game.add_player_effect(effect)
    }
}

/// Change a supported local-player value. Returns false when that value is not
/// implemented or the local player is unavailable.
pub fn adjust_value(game: &mut crate::game::Game<'_>, direction: f32) -> bool {
    match current_page() {
        1 => {
            match item_index() {
                0 => {
                    let Some(player) = game.player.as_mut() else { return false; };
                    let value = (player.health.health() + direction).max(0.0);
                    player.set_health(value, false).is_ok()
                }
                1 => {
                    let Some(player) = game.player.as_mut() else { return false; };
                    player.health.food.food = (player.health.food.food + direction as i32).clamp(0, 20);
                    true
                }
                2 => {
                    let Some(player) = game.player.as_mut() else { return false; };
                    let food = player.health.food.food as f32;
                    player.health.food.saturation = (player.health.food.saturation + direction).clamp(0.0, food);
                    true
                }
                3 => change_attribute(game, "minecraft:max_health", f64::from(direction), 1.0, 2048.0),
                4 => change_attribute(game, "minecraft:attack_damage", f64::from(direction), 0.0, 2048.0),
                5 => change_attribute(game, "minecraft:attack_speed", f64::from(direction) * 0.1, 0.1, 1024.0),
                6 => change_attribute(game, "minecraft:armor", f64::from(direction), 0.0, 30.0),
                7 => change_attribute(game, "minecraft:armor_toughness", f64::from(direction), 0.0, 20.0),
                8 => change_attribute(game, "minecraft:knockback_resistance", f64::from(direction) * 0.05, 0.0, 1.0),
                _ => false,
            }
        }
        2 => change_attribute(game, "minecraft:movement_speed", f64::from(direction) * 0.01, 0.001, 1.0),
        3 => {
            game.world.day_time = (game.world.day_time + (direction as i64 * 1000)).max(0);
            true
        }
        4 => toggle_effect(game, item_index()),
        _ => false,
    }
}

pub fn player_values(game: &crate::game::Game<'_>) -> [String; 9] {
    use horizon_world::attribute_map::ValueKind;
    let Some(p) = game.player.as_ref() else {
        return std::array::from_fn(|_| "N/A".into());
    };
    let attribute = |id: &str| -> String {
        p.attributes.key(id)
            .and_then(|key| p.attributes.value(Some(key), ValueKind::Base).ok())
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|| "N/A".into())
    };
    [
        format!("{:.1}", p.health.health()),
        format!("{}", p.health.food.food),
        format!("{:.1}", p.health.food.saturation),
        attribute("minecraft:max_health"),
        attribute("minecraft:attack_damage"),
        attribute("minecraft:attack_speed"),
        attribute("minecraft:armor"),
        attribute("minecraft:armor_toughness"),
        attribute("minecraft:knockback_resistance"),
    ]
}
pub fn world_time(game: &crate::game::Game<'_>) -> String {
    format!("{}", game.world.day_time.rem_euclid(24_000))
}
pub fn effect_rows(game: &crate::game::Game<'_>) -> [(String, bool); 5] {
    [
        ("Speed".into(), effect_is_active(game, 0)),
        ("Strength".into(), effect_is_active(game, 1)),
        ("Jump Boost".into(), effect_is_active(game, 2)),
        ("Regeneration".into(), effect_is_active(game, 3)),
        ("Invisibility".into(), effect_is_active(game, 4)),
    ]
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

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MENU_OPEN: AtomicBool = AtomicBool::new(false);
static SELECTED_INDEX: AtomicUsize = AtomicUsize::new(0);
static CURRENT_PAGE: AtomicUsize = AtomicUsize::new(0);
static ITEM_INDEX: AtomicUsize = AtomicUsize::new(0);
static EDITING: AtomicBool = AtomicBool::new(false);
static EDIT_BUFFER: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
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
pub fn select_index(index: usize) {
    SELECTED_INDEX.store(index.min(CATEGORY_COUNT - 1), Ordering::Relaxed);
}
pub fn select_next() {
    if current_page() == 0 {
        SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some((v + 1) % CATEGORY_COUNT)).ok();
    } else {
        let count = match current_page() { 1 => PLAYER_ITEM_COUNT, 2 | 3 => 1, 4 => EFFECT_IDS.len(), _ => 1 };
        ITEM_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some((v + 1) % count)).ok();
    }
}
pub fn select_previous() {
    if current_page() == 0 {
        SELECTED_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some((v + CATEGORY_COUNT - 1) % CATEGORY_COUNT)).ok();
    } else {
        let count = match current_page() { 1 => PLAYER_ITEM_COUNT, 2 | 3 => 1, 4 => EFFECT_IDS.len(), _ => 1 };
        ITEM_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some((v + count - 1) % count)).ok();
    }
}
pub fn enter_category() {
    CURRENT_PAGE.store(selected_index() + 1, Ordering::Relaxed);
    ITEM_INDEX.store(0, Ordering::Relaxed);
}
pub fn back_to_categories() {
    if is_editing() { cancel_edit(); }
    CURRENT_PAGE.store(0, Ordering::Relaxed);
    SELECTED_INDEX.store(0, Ordering::Relaxed);
}
fn effect_is_active(_game: &crate::game::Game<'_>, _index: usize) -> bool { false }
pub fn adjust_value(_game: &mut crate::game::Game<'_>, _delta: f64) -> bool { false }

pub fn is_editing() -> bool { EDITING.load(Ordering::Relaxed) }
pub fn edit_buffer() -> String {
    EDIT_BUFFER.lock().map(|s| s.clone()).unwrap_or_default()
}
pub fn begin_edit() -> bool {
    if !matches!(current_page(), 1 | 2 | 3) || (current_page() == 3 && item_index() >= 1) {
        return false;
    }
    if let Ok(mut buffer) = EDIT_BUFFER.lock() { buffer.clear(); }
    EDITING.store(true, Ordering::Relaxed);
    true
}
pub fn cancel_edit() {
    EDITING.store(false, Ordering::Relaxed);
    if let Ok(mut buffer) = EDIT_BUFFER.lock() { buffer.clear(); }
}
pub fn append_edit_char(ch: char) {
    if !is_editing() { return; }
    if let Ok(mut buffer) = EDIT_BUFFER.lock() {
        if buffer.len() < 32 { buffer.push(ch); }
    }
}
pub fn edit_backspace() {
    if let Ok(mut buffer) = EDIT_BUFFER.lock() { buffer.pop(); }
}
fn parse_numeric_input(input: &str) -> Option<f64> {
    match input.trim().to_ascii_lowercase().as_str() {
        "nan" | "+nan" | "-nan" => Some(f64::NAN),
        "inf" | "+inf" | "infinity" | "+infinity" => Some(f64::INFINITY),
        "-inf" | "-infinity" => Some(f64::NEG_INFINITY),
        value => value.parse::<f64>().ok(),
    }
}

pub fn commit_edit(game: &mut crate::game::Game<'_>) -> Result<(), String> {
    let input = edit_buffer();
    let value = parse_numeric_input(&input)
        .ok_or_else(|| "Enter a number, NaN, Infinity or -Infinity".to_string())?;
    let page = current_page();
    let index = item_index();

    // Apply edits on the integrated server thread. Editing only the client copy
    // is overwritten by the authoritative single-player server on the next tick.
    let command = match page {
        1 => match index {
            0 => format!("data modify entity @a[limit=1] Health set value {}f", input.trim()),
            1 => {
                if !value.is_finite() { return Err("Food requires a finite integer".into()); }
                format!("data modify entity @a[limit=1] foodLevel set value {}", value.round() as i32)
            }
            2 => format!("data modify entity @a[limit=1] foodSaturationLevel set value {}f", input.trim()),
            3 => format!("attribute @a[limit=1] minecraft:max_health base set {}", input.trim()),
            4 => format!("attribute @a[limit=1] minecraft:attack_damage base set {}", input.trim()),
            5 => format!("attribute @a[limit=1] minecraft:attack_speed base set {}", input.trim()),
            6 => format!("attribute @a[limit=1] minecraft:armor base set {}", input.trim()),
            7 => format!("attribute @a[limit=1] minecraft:armor_toughness base set {}", input.trim()),
            8 => format!("attribute @a[limit=1] minecraft:knockback_resistance base set {}", input.trim()),
            _ => return Err("This value cannot be edited".into()),
        },
        2 => format!("attribute @a[limit=1] minecraft:movement_speed base set {}", input.trim()),
        3 => {
            if !value.is_finite() { return Err("World time requires a finite integer".into()); }
            format!("time set {}", value as i64)
        }
        _ => return Err("This value cannot be edited".into()),
    };

    let Some(server) = game.integrated.as_ref() else {
        return Err("The integrated single-player server is unavailable".into());
    };
    match server.call(move |server_game| server_game.run_command_quietly(&command)) {
        Some(true) => {}
        Some(false) => return Err("Horizon rejected this value or could not find the player".into()),
        None => return Err("The integrated server did not answer".into()),
    }
    cancel_edit();
    Ok(())
}

pub fn player_values(game: &mut crate::game::Game<'_>) -> [String; 9] {
    use horizon_world::attribute_map::ValueKind;
    let Some(p) = game.player.as_mut() else {
        return std::array::from_fn(|_| "N/A".into());
    };
    let health = format!("{:.1}", p.health.health());
    let food = format!("{}", p.health.food.food);
    let saturation = format!("{:.1}", p.health.food.saturation);
    let mut attribute = |id: &str| -> String {
        let Some(key) = p.attributes.key(id) else { return "N/A".into(); };
        p.attributes.value(Some(key), ValueKind::Base)
            .map(|v| format!("{v:.2}"))
            .unwrap_or_else(|_| "N/A".into())
    };
    [
        health,
        food,
        saturation,
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

pub fn movement_value(game: &mut crate::game::Game<'_>) -> String {
    use horizon_world::attribute_map::ValueKind;
    let Some(player) = game.player.as_mut() else { return "N/A".into(); };
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

fn draw_overlay(
    game: &mut crate::game::Game<'_>,
    mut batch: HudBatch,
) -> HudBatch {
    use crate::game::xelement_value_editor as editor;

    if game.integrated.is_none() || !editor::menu_is_open() {
        return batch;
    }

    let atlas = &game.gui.atlas;
    let white = horizon_client::hud::rgba(245, 248, 255, 255);
    let muted = horizon_client::hud::rgba(175, 190, 210, 255);
    let accent = horizon_client::hud::rgba(255, 220, 90, 255);

    batch.fill(atlas, 16.0, 16.0, 326.0, 270.0, 0xE6111724);
    batch.fill(atlas, 16.0, 16.0, 326.0, 26.0, 0xFF263B58);
    batch.text(atlas, "XElement Value Editor", 26.0, 23.0, white, true);

    if editor::current_page() == 0 {
        batch.text(atlas, "INSERT: close", 230.0, 23.0, muted, true);
        let categories = [
            ("PLAYER", "health / food / attributes"),
            ("MOVEMENT", "movement speed"),
            ("WORLD", "time of day"),
            ("ENTITIES & EFFECTS", "local player potion effects"),
        ];
        let selected = editor::selected_index().min(3);
        for (index, (name, description)) in categories.iter().enumerate() {
            let y = 54.0 + index as f32 * 46.0;
            if index == selected {
                batch.fill(atlas, 24.0, y - 4.0, 310.0, 40.0, 0xFF344B68);
                batch.fill(atlas, 24.0, y - 4.0, 3.0, 40.0, 0xFFFFDC5A);
            }
            let number = match index { 0 => "[1]", 1 => "[2]", 2 => "[3]", _ => "[4]" };
            batch.text(atlas, number, 34.0, y, accent, true);
            batch.text(atlas, name, 70.0, y, white, true);
            batch.text(atlas, description, 70.0, y + 17.0, muted, true);
        }
        batch.fill(atlas, 24.0, 246.0, 310.0, 1.0, 0xFF52647A);
        batch.text(atlas, "ENTER: open   UP/DOWN: select   1-4: jump", 27.0, 254.0, white, true);
        return batch;
    }

    batch.text(atlas, "ESC: back", 260.0, 23.0, muted, true);
    let page = editor::current_page();
    let selected = editor::item_index();

    if page == 1 {
        batch.text(atlas, "PLAYER", 26.0, 49.0, accent, true);
        let values = editor::player_values(game);
        let rows = [
            ("Health", values[0].as_str()),
            ("Food", values[1].as_str()),
            ("Saturation", values[2].as_str()),
            ("Max Health", values[3].as_str()),
            ("Attack Damage", values[4].as_str()),
            ("Attack Speed", values[5].as_str()),
            ("Armor", values[6].as_str()),
            ("Armor Toughness", values[7].as_str()),
            ("Knockback Resist.", values[8].as_str()),
        ];
        for (i, (name, value)) in rows.iter().enumerate() {
            let y = 72.0 + i as f32 * 19.0;
            if i == selected {
                batch.fill(atlas, 24.0, y - 3.0, 310.0, 18.0, 0xFF344B68);
                batch.fill(atlas, 24.0, y - 3.0, 3.0, 18.0, 0xFFFFDC5A);
            }
            batch.text(atlas, name, 38.0, y, white, true);
            batch.text(atlas, value, 260.0, y, accent, true);
        }
        batch.text(atlas, "LEFT/RIGHT: adjust selected value", 27.0, 254.0, white, true);
    } else if page == 2 {
        batch.text(atlas, "MOVEMENT", 26.0, 49.0, accent, true);
        batch.fill(atlas, 24.0, 78.0, 310.0, 30.0, 0xFF344B68);
        batch.text(atlas, "Movement Speed", 38.0, 87.0, white, true);
        let value = editor::movement_value(game);
        batch.text(atlas, &value, 260.0, 87.0, accent, true);
        batch.text(atlas, "LEFT/RIGHT: adjust speed", 27.0, 254.0, white, true);
    } else if page == 3 {
        batch.text(atlas, "WORLD", 26.0, 49.0, accent, true);
        let time = editor::world_time(game);
        batch.fill(atlas, 24.0, 78.0, 310.0, 30.0, 0xFF344B68);
        batch.text(atlas, "Time of day (ticks)", 38.0, 87.0, white, true);
        batch.text(atlas, &time, 260.0, 87.0, accent, true);
        batch.text(atlas, "LEFT/RIGHT: -/+ 1000 ticks", 27.0, 254.0, white, true);
    } else if page == 4 {
        batch.text(atlas, "PLAYER EFFECTS", 26.0, 49.0, accent, true);
        let rows = editor::effect_rows(game);
        for (i, (name, active)) in rows.iter().enumerate() {
            let y = 82.0 + i as f32 * 30.0;
            if i == selected {
                batch.fill(atlas, 24.0, y - 5.0, 310.0, 26.0, 0xFF344B68);
                batch.fill(atlas, 24.0, y - 5.0, 3.0, 26.0, 0xFFFFDC5A);
            }
            batch.text(atlas, name, 38.0, y, white, true);
            batch.text(atlas, if *active { "ON" } else { "OFF" }, 270.0, y, accent, true);
        }
        batch.text(atlas, "LEFT/RIGHT: toggle effect", 27.0, 254.0, white, true);
    }

    batch
}

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

    // Main panel and title.
    batch.fill(atlas, 16.0, 16.0, 326.0, 226.0, 0xE6111724);
    batch.fill(atlas, 16.0, 16.0, 326.0, 26.0, 0xFF263B58);
    batch.text(atlas, "XElement Value Editor", 26.0, 23.0, white, true);
    batch.text(atlas, "INSERT: close", 230.0, 23.0, muted, true);

    let categories = [
        ("PLAYER", "health / max health / food"),
        ("MOVEMENT", "speed / gravity / jump"),
        ("WORLD", "time / weather / game speed"),
        ("ENTITIES & EFFECTS", "health / size / duration"),
    ];
    let selected = editor::selected_index().min(3);

    for (index, (name, description)) in categories.iter().enumerate() {
        let y = 50.0 + index as f32 * 39.0;
        if index == selected {
            batch.fill(atlas, 24.0, y - 4.0, 310.0, 34.0, 0xFF344B68);
            batch.fill(atlas, 24.0, y - 4.0, 3.0, 34.0, 0xFFFFDC5A);
        }
        let number = match index {
            0 => "[1]",
            1 => "[2]",
            2 => "[3]",
            _ => "[4]",
        };
        batch.text(atlas, number, 34.0, y, accent, true);
        batch.text(atlas, name, 70.0, y, white, true);
        batch.text(atlas, description, 70.0, y + 14.0, muted, true);
    }

    batch.fill(atlas, 24.0, 213.0, 310.0, 1.0, 0xFF52647A);
    batch.text(atlas, "1-4: select category   UP/DOWN: move", 27.0, 220.0, white, true);

    batch
}

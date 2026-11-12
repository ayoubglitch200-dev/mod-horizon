fn draw_overlay(
    game: &mut crate::game::Game<'_>,
    mut batch: horizon_client::hud::HudBatch,
) -> HudBatch {
    use crate::game::xelement_value_editor as editor;

    if game.remote.is_some() || !editor::menu_is_open() {
        return batch;
    }

    let atlas = &game.gui.atlas;
    batch.fill(atlas, 16.0, 16.0, 300.0, 198.0, 0xE6111724);
    batch.fill(atlas, 16.0, 16.0, 300.0, 24.0, 0xFF263B58);
    batch.text(
        atlas,
        "XElement Value Editor  |  INSERT: close",
        24.0,
        23.0,
        horizon_client::hud::rgba(255, 255, 255, 255),
        true,
    );

    let lines = [
        ("PLAYER", 50.0, 112, 200, 255),
        ("health / max_health / food / saturation", 66.0, 255, 255, 255),
        ("MOVEMENT", 88.0, 112, 200, 255),
        ("speed / gravity / jump / fall_speed", 104.0, 255, 255, 255),
        ("WORLD", 126.0, 112, 200, 255),
        ("time / weather / game_speed", 142.0, 255, 255, 255),
        ("ENTITIES & EFFECTS", 164.0, 112, 200, 255),
        ("health / size / duration / amplifier", 180.0, 255, 255, 255),
    ];

    for (text, y, r, g, b) in lines {
        batch.text(atlas, text, 26.0, y, horizon_client::hud::rgba(r, g, b, 255), true);
    }

    batch
}

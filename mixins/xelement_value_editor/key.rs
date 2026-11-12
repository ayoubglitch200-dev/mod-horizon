fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) {
    // In Horizon, a single-player client also has a Remote connection to its
    // integrated server. The presence of remote therefore does NOT mean
    // multiplayer. integrated is present only for a local single-player world.
    if game.integrated.is_none() {
        return;
    }

    if key == winit::keyboard::KeyCode::Insert || raw == winit::keyboard::KeyCode::Insert {
        crate::game::xelement_value_editor::toggle_menu();
    }
}

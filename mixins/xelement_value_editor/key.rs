fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) {
    if game.remote.is_some() {
        return;
    }
    if key == winit::keyboard::KeyCode::Insert || raw == winit::keyboard::KeyCode::Insert {
        crate::game::xelement_value_editor::toggle_menu();
    }
}

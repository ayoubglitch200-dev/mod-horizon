fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) -> core::ops::ControlFlow<Result<()>> {
    use core::ops::ControlFlow;
    use winit::keyboard::KeyCode;

    // The editor is intentionally limited to a local integrated-server world.
    if game.integrated.is_none() {
        return ControlFlow::Continue(());
    }

    let pressed = if key == KeyCode::Unidentified(winit::keyboard::NativeKeyCode::Unidentified) {
        raw
    } else {
        key
    };

    if pressed == KeyCode::Insert || raw == KeyCode::Insert {
        crate::game::xelement_value_editor::toggle_menu();
        return ControlFlow::Break(Ok(()));
    }

    if !crate::game::xelement_value_editor::menu_is_open() {
        return ControlFlow::Continue(());
    }

    use crate::game::xelement_value_editor as editor;
    match pressed {
        KeyCode::ArrowDown | KeyCode::KeyS => editor::select_next(),
        KeyCode::ArrowUp | KeyCode::KeyW => editor::select_previous(),
        KeyCode::Escape => editor::close_menu(),
        // Prevent normal gameplay movement while the editor is open.
        KeyCode::ArrowLeft | KeyCode::ArrowRight
        | KeyCode::KeyA | KeyCode::KeyD
        | KeyCode::Space | KeyCode::Enter => {},
        _ => return ControlFlow::Continue(()),
    }

    ControlFlow::Break(Ok(()))
}

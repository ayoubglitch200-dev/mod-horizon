fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) -> core::ops::ControlFlow<Result<()>> {
    use core::ops::ControlFlow;
    use winit::keyboard::KeyCode;

    // Only run in a local single-player world.
    if game.integrated.is_none() {
        return ControlFlow::Continue(());
    }

    if key == KeyCode::Insert || raw == KeyCode::Insert {
        crate::game::xelement_value_editor::toggle_menu();
        // Show the system cursor while the overlay is open; recapture it when closed.
        game.capture(!crate::game::xelement_value_editor::menu_is_open());
        return ControlFlow::Break(Ok(()));
    }

    if !crate::game::xelement_value_editor::menu_is_open() {
        return ControlFlow::Continue(());
    }

    use crate::game::xelement_value_editor as editor;
    let pressed = key;
    let raw_pressed = raw;

    // Number keys select a category directly, without relying on navigation.
    let direct_selection = match (pressed, raw_pressed) {
        (KeyCode::Digit1, _) | (_, KeyCode::Digit1) => Some(0),
        (KeyCode::Digit2, _) | (_, KeyCode::Digit2) => Some(1),
        (KeyCode::Digit3, _) | (_, KeyCode::Digit3) => Some(2),
        (KeyCode::Digit4, _) | (_, KeyCode::Digit4) => Some(3),
        _ => None,
    };
    if let Some(index) = direct_selection {
        editor::select_index(index);
        return ControlFlow::Break(Ok(()));
    }

    if pressed == KeyCode::ArrowDown || raw_pressed == KeyCode::ArrowDown
        || pressed == KeyCode::KeyS || raw_pressed == KeyCode::KeyS {
        editor::select_next();
    } else if pressed == KeyCode::ArrowUp || raw_pressed == KeyCode::ArrowUp
        || pressed == KeyCode::KeyW || raw_pressed == KeyCode::KeyW {
        editor::select_previous();
    } else if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape {
        editor::close_menu();
        game.capture(true);
    } else if pressed == KeyCode::ArrowLeft || raw_pressed == KeyCode::ArrowLeft
        || pressed == KeyCode::ArrowRight || raw_pressed == KeyCode::ArrowRight
        || pressed == KeyCode::KeyA || raw_pressed == KeyCode::KeyA
        || pressed == KeyCode::KeyD || raw_pressed == KeyCode::KeyD
        || pressed == KeyCode::Space || raw_pressed == KeyCode::Space
        || pressed == KeyCode::Enter || raw_pressed == KeyCode::Enter {
        // These keys belong to the editor while it is open, not to gameplay.
    } else {
        return ControlFlow::Continue(());
    }

    // Skip Game::key_pressed so movement and other gameplay actions don't run.
    ControlFlow::Break(Ok(()))
}

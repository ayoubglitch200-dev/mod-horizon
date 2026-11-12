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


fn mouse_category_click(
    game: &mut crate::game::Game<'_>,
    event: &winit::event::WindowEvent,
) -> core::ops::ControlFlow<()> {
    use core::ops::ControlFlow;
    use winit::event::{ElementState, MouseButton, WindowEvent};

    use crate::game::xelement_value_editor as editor;

    if game.integrated.is_none() || !editor::menu_is_open() {
        return ControlFlow::Continue(());
    }

    if let WindowEvent::MouseInput {
        state: ElementState::Pressed,
        button: MouseButton::Left,
        ..
    } = event
    {
        let (x, y) = game.cursor;
        if (16.0..=316.0).contains(&x) {
            let selected = if (40.0..=78.0).contains(&y) {
                Some(0)
            } else if (78.0..=116.0).contains(&y) {
                Some(1)
            } else if (116.0..=154.0).contains(&y) {
                Some(2)
            } else if (154.0..=192.0).contains(&y) {
                Some(3)
            } else {
                None
            };

            if let Some(index) = selected {
                editor::select_index(index);
            }
        }
    }

    // While the overlay is open, mouse clicks must not reach the game world.
    if matches!(event, WindowEvent::MouseInput { .. }) {
        ControlFlow::Break(())
    } else {
        ControlFlow::Continue(())
    }
}

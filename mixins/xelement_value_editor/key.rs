fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) -> core::ops::ControlFlow<Result<()>> {
    use core::ops::ControlFlow;
    use winit::keyboard::KeyCode;
    use crate::game::xelement_value_editor as editor;

    if game.integrated.is_none() {
        return ControlFlow::Continue(());
    }

    if key == KeyCode::Insert || raw == KeyCode::Insert {
        editor::toggle_menu();
        game.capture(!editor::menu_is_open());
        return ControlFlow::Break(Ok(()));
    }
    if !editor::menu_is_open() {
        return ControlFlow::Continue(());
    }

    let pressed = key;
    let raw_pressed = raw;

    if editor::current_page() == 0 {
        let direct = match (pressed, raw_pressed) {
            (KeyCode::Digit1, _) | (_, KeyCode::Digit1) => Some(0),
            (KeyCode::Digit2, _) | (_, KeyCode::Digit2) => Some(1),
            (KeyCode::Digit3, _) | (_, KeyCode::Digit3) => Some(2),
            (KeyCode::Digit4, _) | (_, KeyCode::Digit4) => Some(3),
            _ => None,
        };
        if let Some(index) = direct {
            editor::select_index(index);
            return ControlFlow::Break(Ok(()));
        }
        if pressed == KeyCode::Enter || raw_pressed == KeyCode::Enter {
            editor::enter_category();
        } else if pressed == KeyCode::ArrowDown || raw_pressed == KeyCode::ArrowDown
            || pressed == KeyCode::KeyS || raw_pressed == KeyCode::KeyS {
            editor::select_next();
        } else if pressed == KeyCode::ArrowUp || raw_pressed == KeyCode::ArrowUp
            || pressed == KeyCode::KeyW || raw_pressed == KeyCode::KeyW {
            editor::select_previous();
        } else if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape {
            editor::close_menu();
            game.capture(true);
        } else {
            return ControlFlow::Continue(());
        }
    } else {
        if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape
            || pressed == KeyCode::Backspace || raw_pressed == KeyCode::Backspace {
            editor::back_to_categories();
        } else if pressed == KeyCode::ArrowDown || raw_pressed == KeyCode::ArrowDown
            || pressed == KeyCode::KeyS || raw_pressed == KeyCode::KeyS {
            editor::select_next();
        } else if pressed == KeyCode::ArrowUp || raw_pressed == KeyCode::ArrowUp
            || pressed == KeyCode::KeyW || raw_pressed == KeyCode::KeyW {
            editor::select_previous();
        } else if pressed == KeyCode::ArrowRight || raw_pressed == KeyCode::ArrowRight
            || pressed == KeyCode::KeyD || raw_pressed == KeyCode::KeyD {
            let _ = editor::adjust_value(game, 1.0);
        } else if pressed == KeyCode::ArrowLeft || raw_pressed == KeyCode::ArrowLeft
            || pressed == KeyCode::KeyA || raw_pressed == KeyCode::KeyA {
            let _ = editor::adjust_value(game, -1.0);
        } else {
            return ControlFlow::Continue(());
        }
    }

    ControlFlow::Break(Ok(()))
}

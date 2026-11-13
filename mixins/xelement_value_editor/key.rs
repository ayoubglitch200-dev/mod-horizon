fn toggle_insert(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    raw: winit::keyboard::KeyCode,
) -> core::ops::ControlFlow<Result<()>> {
    use core::ops::ControlFlow;
    use winit::keyboard::KeyCode;
    use crate::game::xelement_value_editor as editor;

    if game.integrated.is_none() { return ControlFlow::Continue(()); }
    if key == KeyCode::Insert || raw == KeyCode::Insert {
        if editor::is_editing() { editor::cancel_edit(); }
        editor::toggle_menu();
        game.capture(!editor::menu_is_open());
        return ControlFlow::Break(Ok(()));
    }
    if !editor::menu_is_open() { return ControlFlow::Continue(()); }

    let pressed = key;
    let raw_pressed = raw;
    if editor::is_editing() {
        if pressed == KeyCode::Enter || raw_pressed == KeyCode::Enter {
            let _ = editor::commit_edit(game);
            return ControlFlow::Break(Ok(()));
        }
        if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape {
            editor::cancel_edit();
            return ControlFlow::Break(Ok(()));
        }
        if pressed == KeyCode::Backspace || raw_pressed == KeyCode::Backspace {
            editor::edit_backspace();
            return ControlFlow::Break(Ok(()));
        }
        let ch = match pressed {
            KeyCode::Digit0 => Some('0'), KeyCode::Digit1 => Some('1'),
            KeyCode::Digit2 => Some('2'), KeyCode::Digit3 => Some('3'),
            KeyCode::Digit4 => Some('4'), KeyCode::Digit5 => Some('5'),
            KeyCode::Digit6 => Some('6'), KeyCode::Digit7 => Some('7'),
            KeyCode::Digit8 => Some('8'), KeyCode::Digit9 => Some('9'),
            KeyCode::Numpad0 => Some('0'), KeyCode::Numpad1 => Some('1'),
            KeyCode::Numpad2 => Some('2'), KeyCode::Numpad3 => Some('3'),
            KeyCode::Numpad4 => Some('4'), KeyCode::Numpad5 => Some('5'),
            KeyCode::Numpad6 => Some('6'), KeyCode::Numpad7 => Some('7'),
            KeyCode::Numpad8 => Some('8'), KeyCode::Numpad9 => Some('9'),
            KeyCode::KeyN => Some('n'), KeyCode::KeyA => Some('a'),
            KeyCode::KeyI => Some('i'), KeyCode::KeyF => Some('f'),
            KeyCode::KeyT => Some('t'), KeyCode::KeyY => Some('y'),
            KeyCode::KeyE => Some('e'), KeyCode::Minus | KeyCode::NumpadSubtract => Some('-'),
            KeyCode::Equal | KeyCode::NumpadAdd => Some('+'),
            KeyCode::Period | KeyCode::NumpadDecimal => Some('.'),
            _ => None,
        };
        if let Some(ch) = ch { editor::append_edit_char(ch); }
        return ControlFlow::Break(Ok(()));
    }

    if editor::current_page() == 0 {
        let direct = match (pressed, raw_pressed) {
            (KeyCode::Digit1, _) | (_, KeyCode::Digit1) => Some(0),
            (KeyCode::Digit2, _) | (_, KeyCode::Digit2) => Some(1),
            (KeyCode::Digit3, _) | (_, KeyCode::Digit3) => Some(2),
            (KeyCode::Digit4, _) | (_, KeyCode::Digit4) => Some(3),
            _ => None,
        };
        if let Some(index) = direct { editor::select_index(index); return ControlFlow::Break(Ok(())); }
        if pressed == KeyCode::Enter || raw_pressed == KeyCode::Enter { editor::enter_category(); }
        else if pressed == KeyCode::ArrowDown || raw_pressed == KeyCode::ArrowDown || pressed == KeyCode::KeyS || raw_pressed == KeyCode::KeyS { editor::select_next(); }
        else if pressed == KeyCode::ArrowUp || raw_pressed == KeyCode::ArrowUp || pressed == KeyCode::KeyW || raw_pressed == KeyCode::KeyW { editor::select_previous(); }
        else if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape { editor::close_menu(); game.capture(true); }
        else { return ControlFlow::Continue(()); }
    } else {
        if pressed == KeyCode::Escape || raw_pressed == KeyCode::Escape || pressed == KeyCode::Backspace || raw_pressed == KeyCode::Backspace { editor::back_to_categories(); }
        else if pressed == KeyCode::Enter || raw_pressed == KeyCode::Enter { let _ = editor::begin_edit(); }
        else if pressed == KeyCode::ArrowDown || raw_pressed == KeyCode::ArrowDown || pressed == KeyCode::KeyS || raw_pressed == KeyCode::KeyS { editor::select_next(); }
        else if pressed == KeyCode::ArrowUp || raw_pressed == KeyCode::ArrowUp || pressed == KeyCode::KeyW || raw_pressed == KeyCode::KeyW { editor::select_previous(); }
        else if pressed == KeyCode::ArrowRight || raw_pressed == KeyCode::ArrowRight { let _ = editor::adjust_value(game, 1.0); }
        else if pressed == KeyCode::ArrowLeft || raw_pressed == KeyCode::ArrowLeft { let _ = editor::adjust_value(game, -1.0); }
        else { return ControlFlow::Continue(()); }
    }
    ControlFlow::Break(Ok(()))
}

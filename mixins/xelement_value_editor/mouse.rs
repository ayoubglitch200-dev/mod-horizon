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

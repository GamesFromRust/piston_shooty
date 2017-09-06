use std::collections::HashMap;
use input;
use piston_window::MouseButton;

pub fn did_click(mouse_states: &HashMap<MouseButton, input::ButtonState>) -> bool {
    match mouse_states.get(&MouseButton::Left) {
        Some(value) => {
            if value.pressed {
                return true;
            }
        },
        _ => {}
    }
    false
}

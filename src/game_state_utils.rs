use std::collections::HashMap;
use crate::input;
use piston_window::MouseButton;
use piston_window::Key;

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

pub fn did_press_key(key_states: &HashMap<Key, input::ButtonState>, key: Key) -> bool {
    match key_states.get(&key) {
        Some(value) => {
            if value.pressed {
                return true;
            }
        },
        _ => {}
    }
    false
}

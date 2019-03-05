use std::collections::HashMap;
use crate::input;
use piston_window::MouseButton;
use piston_window::Key;

pub fn did_click(mouse_states: &HashMap<MouseButton, input::ButtonState>) -> bool {
    if let Some(value) = mouse_states.get(&MouseButton::Left) {
        if value.pressed {
            return true;
        }
    }
    false
}

pub fn did_press_key(key_states: &HashMap<Key, input::ButtonState>, key: Key) -> bool {
    if let Some(value) = key_states.get(&key) {
        if value.pressed {
            return true;
        }
    }
    false
}

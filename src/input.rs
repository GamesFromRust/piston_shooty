use piston_window::*;
use std::collections::HashMap;

use crate::vector2::*;

// TODO: Make singleton.

#[derive(Clone, Copy)]
pub struct ButtonState {
    // "Pressed" for 2+ frames.
    pub held: bool,
    // Pressed for 1 frame.
    pub pressed: bool,
    // Released for 1 frame.
    pub released: bool,
}

pub fn gather_input(input: &Event, key_states: &mut HashMap<Key, ButtonState>, mouse_states: &mut HashMap<MouseButton, ButtonState>, mouse_pos: &mut Vector2) {
    // Keyboard pressed.
    if let Some(Button::Keyboard(key)) = input.press_args() {
        // If the key is being held, we want to ignore this pressed event.
        if let Some(ks) = key_states.get_mut(&key) {
            if ks.held {
                return;
            }
        }

        // Otherwise, insert the key state.
        let key_state = ButtonState {
            held: false,
            pressed: true,
            released: false,
        };
        key_states.insert(key, key_state);
    }

    // Keyboard released.
    if let Some(Button::Keyboard(key)) = input.release_args() {
        // Insert the released key state.
        let key_state = ButtonState {
            held: false,
            pressed: false,
            released: true,
        };
        key_states.insert(key, key_state);
    }

    // Mouse pressed.
    if let Some(Button::Mouse(key)) = input.press_args() {
        let key_state = ButtonState {
            held: false,
            pressed: true,
            released: false,
        };
        mouse_states.insert(key, key_state);
    }

    // Mouse released.
    if let Some(Button::Mouse(key)) = input.release_args() {
        let key_state = ButtonState {
            held: false,
            pressed: false,
            released: true,
        };
        mouse_states.insert(key, key_state);
    }

    // Mouse moved.
    if let Some(pos) = input.mouse_cursor_args() {
        mouse_pos.x = pos[0];
        mouse_pos.y = pos[1];
    }
}

pub fn update_input(key_states: &mut HashMap<Key, ButtonState>, mouse_states: &mut HashMap<MouseButton, ButtonState>) {
    for value in key_states.values_mut() {
        if value.pressed {
            // If we're pressed, change state to held.
            value.held = true;
            value.pressed = false;
        } else if value.released {
            // If we were just released, reset it.
            value.released = false;
        }
    }

    for value in mouse_states.values_mut() {
        if value.pressed {
            // If we're pressed, change state to held.
            value.held = true;
            value.pressed = false;
        } else if value.released {
            // If we were just released, reset it.
            value.released = false;
        }
    }
}

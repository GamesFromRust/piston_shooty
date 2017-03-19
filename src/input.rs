use piston_window::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct KeyState {
    // "Pressed" for 2+ frames.
    pub held: bool,
    // Pressed for 1 frame.
    pub pressed: bool,
    // Released for 1 frame.
    pub released: bool,
}

pub fn gather_input(input: &Input, key_states:&mut HashMap<Key, KeyState>) {
    if let Some(Button::Keyboard(key)) = input.press_args() {
        // If the key is being held, we want to ignore this pressed event.
        if let Some(ks) = key_states.get_mut(&key) {
            if ks.held {
                return
            }
        }

        // Otherwise, insert the key state.
        let key_state = KeyState { held: false, pressed: true, released: false };
        key_states.insert(key, key_state);
    }

    if let Some(Button::Keyboard(key)) = input.release_args() {
        // Insert the released key state.
        let key_state = KeyState { held: false, pressed: false, released: true };
        key_states.insert(key, key_state);
    }
}

pub fn update_input(key_states: &mut HashMap<Key, KeyState>) {
     for (_, value) in key_states {
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

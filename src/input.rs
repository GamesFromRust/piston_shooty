use piston_window::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct KeyState {
    pub held: bool,
    pub pressed: bool,
    pub released: bool,
}

pub fn gather_input(input: &Input, key_states:&mut HashMap<Key, KeyState>) {    
    if let Some(Button::Keyboard(key)) = input.press_args() {
        let mut key_state = KeyState { held: false, pressed: true, released: false };
        if let Some(ks) = key_states.get_mut(&key) {
            if ks.pressed {
                key_state.held = true;
            }
        }

        println!("ks, held: {0}, pressed: {1}, released: {2} ", key_state.held, key_state.pressed, key_state.released);

        key_states.insert(key, key_state);
    }
    // if let Some(u) = input.update_args() {
    //     let mut key_state = KeyState { held: false, pressed: true, released: false };
    //     if let Some(ks) = key_states.get_mut(&key) {
    //         if ks.pressed {
    //             key_state.held = true;
    //         }
    //     }
    //     println!("ks, held: {0}, pressed: {1}, released: {2} ", key_state.held, key_state.pressed, key_state.released);

    //     key_states.insert(key, key_state);
    // }
    if let Some(Button::Keyboard(key)) = input.release_args() {
        let key_state = KeyState { held: false, pressed: false, released: true };

        println!("ks, held: {0}, pressed: {1}, released: {2} ", key_state.held, key_state.pressed, key_state.released);
        key_states.insert(key, key_state);
    }
}
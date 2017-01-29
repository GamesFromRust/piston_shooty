// use piston::window::WindowSettings;
// use piston::event_loop::*;
// use piston::event_loop::*;
// use piston::event_loop::WindowEvents;
use piston_window::*;
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::HashMap;
// use std::ops::*;

#[derive(Clone, Copy)]
pub struct KeyState {
    pub held: bool,
    pub pressed: bool,
    pub released: bool,
}

pub fn gather_input(e: &Event, key_states:&mut HashMap<Key, KeyState>) {    
    if let Some(Button::Keyboard(key)) = e.press_args() {
        let key_state = KeyState { held: false, pressed: true, released: false };
        if let Some(key_state) = key_states.get_mut(&key) {
            if key_state.pressed {
                key_state.held = true;
                key_state.released = false;
            }
        }
        key_states.insert(key, key_state);
    }
    if let Some(Button::Keyboard(key)) = e.release_args() {
        let key_state = KeyState { held: false, pressed: false, released: true };
        if let Some(key_state) = key_states.get_mut(&key) {
            key_state.pressed = false;
            key_state.held = false;
        }
        key_states.insert(key, key_state);
    }
}
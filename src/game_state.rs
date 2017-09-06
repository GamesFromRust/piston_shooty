use font_manager::FontManager;
use piston_window::Context;
use piston_window::G2d;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use input;
use vector2::Vector2;

pub trait GameState {
    fn render(
        &self, 
        c: Context, 
        gl: &mut G2d,
        font_manager: &mut FontManager, 
        window_width: f64,
        window_height: f64);

    fn update(
        &mut self, 
        key_states: &HashMap<Key, input::ButtonState>, 
        mouse_states: &HashMap<MouseButton, input::ButtonState>, 
        mouse_pos: &Vector2, 
        args: &UpdateArgs);
}
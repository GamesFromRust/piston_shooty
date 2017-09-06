use render_utils;
use game_state::GameState;
use game_state::UpdateResult;
use font_manager::FontManager;
use piston_window::Context;
use piston_window::G2d;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use input;
use vector2::Vector2;
use game_state_utils;

pub struct VictoryScreen {

}

impl GameState for VictoryScreen {
    fn render(
        &self, 
        c: Context, 
        mut gl: &mut G2d,
        mut font_manager: &mut FontManager, 
        window_width: f64, 
        window_height: f64) {
        
        render_utils::draw_text_overlay(
            &mut font_manager, 
            &c, 
            &mut gl,
            window_width, 
            window_height, 
            "VICTORY! Click to continue.");
    }

    fn update(
        &mut self, 
        key_states: &HashMap<Key, input::ButtonState>, 
        mouse_states: &HashMap<MouseButton, 
        input::ButtonState>, 
        mouse_pos: &Vector2, 
        args: &UpdateArgs) -> UpdateResult {

        if game_state_utils::did_click(&mouse_states) {
            return UpdateResult::Success;
        } else {
            return UpdateResult::Running;
        }
    }
}
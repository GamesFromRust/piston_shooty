use render_utils;
use game_state::GameState;
use game_state::GameStateType;
use game_state::UpdateResult;
use game_state::UpdateResultType;
use game_state::UPDATE_RESULT_RUNNING;
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
use std::rc::Rc;
use colors;

pub struct MenuScreen<'a> {
    pub world_list: Rc<Vec<&'a str>>,
    pub selected_world_index: usize,
}

impl<'a> GameState for MenuScreen<'a> {
    fn render(
        &self, 
        c: Context, 
        mut gl: &mut G2d,
        mut font_manager: &mut FontManager, 
        window_width: f64, 
        window_height: f64) {
        
        for i in 0..self.world_list.len() {
            let mut color = colors::WHITE;
            if i == self.selected_world_index {
                color = colors::BLUE;
            }
            render_utils::draw_text_overlay(
                &mut font_manager, 
                &c, 
                &mut gl,
                window_width, 
                window_height,
                0.5,
                0.5 + (0.05 * ((i+1) as f64)),
                self.world_list[i], 
                color);
        }

        render_utils::draw_text_overlay(
            &mut font_manager, 
            &c, 
            &mut gl,
            window_width, 
            window_height,
            0.5,
            0.5, 
            "WELCOME TO GUNGUN WARRIORS",
            colors::WHITE);
    }

    #[allow(unused_variables)]
    fn update(
        &mut self, 
        key_states: &HashMap<Key, input::ButtonState>, 
        mouse_states: &HashMap<MouseButton, 
        input::ButtonState>, 
        mouse_pos: &Vector2, 
        args: &UpdateArgs) -> UpdateResult {

        if game_state_utils::did_press_key(&key_states, Key::Up) {
            if self.selected_world_index > 0 {
                self.selected_world_index = self.selected_world_index - 1;
            }
        }

        if game_state_utils::did_press_key(&key_states, Key::Down) {
            if self.selected_world_index < self.world_list.len() - 1 {
                self.selected_world_index = self.selected_world_index + 1;
            }
        }

        if game_state_utils::did_press_key(&key_states, Key::Down) {
            if self.selected_world_index < self.world_list.len() - 1 {
                self.selected_world_index = self.selected_world_index + 1;
            }
        }

        if game_state_utils::did_click(&mouse_states) || game_state_utils::did_press_key(&key_states, Key::Return) {
            return UpdateResult {
                result_type: UpdateResultType::Success,
                result_code: self.selected_world_index as i64,
            }
        } else {
            return UPDATE_RESULT_RUNNING;
        }
    }

    fn get_type(&self) -> GameStateType {
        return GameStateType::WorldSelect;        
    }
}
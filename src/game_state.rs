use font_manager::FontManager;
use piston_window::Context;
use piston_window::G2d;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use input;
use vector2::Vector2;

pub struct UpdateResult {
    pub result_type: UpdateResultType,
    pub result_code: i64,
}

pub const UPDATE_RESULT_RUNNING: UpdateResult = UpdateResult {
    result_type: UpdateResultType::Running,
    result_code: 0,
};

pub const UPDATE_RESULT_SUCCESS: UpdateResult = UpdateResult {
    result_type: UpdateResultType::Success,
    result_code: 0,
};

pub const UPDATE_RESULT_FAIL: UpdateResult = UpdateResult {
    result_type: UpdateResultType::Fail,
    result_code: 0,
};

pub enum UpdateResultType {
    Running,
    Success,
    Fail,
}

#[derive(PartialEq, Eq)]
pub enum GameStateType {
    WorldSelect,
    World,
    Victory
}

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
        args: &UpdateArgs) -> UpdateResult;

    fn get_type(&self) -> GameStateType;
}

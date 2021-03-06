use crate::input;
use crate::ui_bundle::UiBundle;
use crate::vector2::Vector2;
use piston_window::Context;
use piston_window::G2d;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;

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
    Victory,
}

pub trait GameState {
    fn render(&mut self, c: Context, gl: &mut G2d, ui_bundle: &mut UiBundle);

    fn update(
        &mut self,
        key_states: &HashMap<Key, input::ButtonState>,
        mouse_states: &HashMap<MouseButton, input::ButtonState>,
        mouse_pos: &Vector2,
        ui_bundle: &mut UiBundle,
        args: UpdateArgs,
    ) -> UpdateResult;

    fn get_type(&self) -> GameStateType;
}

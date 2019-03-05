use crate::game_state::GameState;
use crate::game_state::GameStateType;
use crate::game_state::UpdateResult;
use crate::game_state::UPDATE_RESULT_RUNNING;
use crate::game_state::UPDATE_RESULT_SUCCESS;
use crate::game_state_utils;
use crate::input;
use crate::render_utils;
use crate::ui_bundle::UiBundle;
use crate::vector2::Vector2;
use conrod_core::color::Colorable;
use conrod_core::widget::Widget;
use piston_window::Context;
use piston_window::G2d;
use piston_window::G2dTexture;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;

pub struct VictoryScreen {
    pub image_map: conrod_core::image::Map<G2dTexture>,
}

impl GameState for VictoryScreen {
    fn render(&mut self, c: Context, gl: &mut G2d, ui_bundle: &mut UiBundle) {
        ui_bundle.render_ui(c, gl, &self.image_map);
    }

    #[allow(unused_variables)]
    fn update(
        &mut self,
        key_states: &HashMap<Key, input::ButtonState>,
        mouse_states: &HashMap<MouseButton, input::ButtonState>,
        mouse_pos: &Vector2,
        ui_bundle: &mut UiBundle,
        args: UpdateArgs,
    ) -> UpdateResult {
        let mut ui_cell = ui_bundle.conrod_ui.set_widgets();

        conrod_core::widget::Canvas::new().pad(30.0).color(conrod_core::color::TRANSPARENT).scroll_kids_vertically().set(ui_bundle.ids.canvas, &mut ui_cell);

        render_utils::draw_text_overlay("VICTORY! Click to continue.", &mut ui_cell, &ui_bundle.ids, conrod_core::color::WHITE, 36);

        if game_state_utils::did_click(&mouse_states) {
            UPDATE_RESULT_SUCCESS
        } else {
            UPDATE_RESULT_RUNNING
        }
    }

    fn get_type(&self) -> GameStateType {
        GameStateType::Victory
    }
}

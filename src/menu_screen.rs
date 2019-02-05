use crate::game_state::GameState;
use crate::game_state::GameStateType;
use crate::game_state::UpdateResult;
use crate::game_state::UpdateResultType;
use crate::game_state::UPDATE_RESULT_RUNNING;
use piston_window;
use piston_window::*;
use std::collections::HashMap;
use crate::input;
use crate::vector2::Vector2;
use crate::game_state_utils;
use std::rc::Rc;
use crate::ui_bundle::UiBundle;
use conrod_core::color::Colorable;
use conrod_core::Widget;
use conrod_core::Positionable;
use conrod_core::image::Id;
use crate::fps_counter::FpsCounter;

pub struct MenuScreen<'a> {
    pub world_list: Rc<Vec<&'a str>>,
    pub selected_world_index: usize,
    pub fps_counter: FpsCounter,
    pub image_map: conrod_core::image::Map<G2dTexture>,
    pub logo_image_id: Id, // todo: remove
}

impl<'a> GameState for MenuScreen<'a> {
    fn render(
        &mut self, 
        c: Context, 
        gl: &mut G2d,
        ui_bundle: &mut UiBundle) {

        self.fps_counter.calculate_fps();

        ui_bundle.render_ui(c, gl, &self.image_map);
    }

    #[allow(unused_variables)]
    fn update(
        &mut self, 
        key_states: &HashMap<Key, input::ButtonState>, 
        mouse_states: &HashMap<MouseButton, input::ButtonState>, 
        mouse_pos: &Vector2, 
        ui_bundle: &mut UiBundle,
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

        self.update_ui(ui_bundle);

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

impl<'a> MenuScreen<'a> {
    fn update_ui(&self, ui_bundle: &mut UiBundle) {
        ui_bundle.ids.world_list.resize(self.world_list.len(), &mut ui_bundle.conrod_ui.widget_id_generator());

        let mut ui_cell = ui_bundle.conrod_ui.set_widgets();

        conrod_core::widget::Canvas::new()
            .pad(30.0)
            .color(conrod_core::color::TRANSPARENT)
            .scroll_kids_vertically()
            .set(ui_bundle.ids.canvas, &mut ui_cell);
        conrod_core::widget::Text::new("WELCOME TO GUNGUN WARRIORS")
            .font_size(36)
            .color(conrod_core::color::WHITE)
            .mid_top_of(ui_bundle.ids.canvas)
            .set(ui_bundle.ids.title, &mut ui_cell);
        
        let mut id_widget_above = ui_bundle.ids.title;
        for i in 0..self.world_list.len() {
            let mut color = conrod_core::color::WHITE;
            if i == self.selected_world_index {
                color = conrod_core::color::BLUE;
            }

            conrod_core::widget::Text::new(self.world_list[i])
                .font_size(36)
                .color(color)
                .down_from(id_widget_above, 5.0)
                .align_middle_x_of(ui_bundle.ids.canvas)
                .set(ui_bundle.ids.world_list[i], &mut ui_cell);
            id_widget_above = ui_bundle.ids.world_list[i];
        }

        self.fps_counter.update_ui(&mut ui_cell, &ui_bundle.ids);
    }
}

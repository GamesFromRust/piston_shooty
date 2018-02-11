use render_utils;
use game_state::GameState;
use game_state::GameStateType;
use game_state::UpdateResult;
use game_state::UpdateResultType;
use game_state::UPDATE_RESULT_RUNNING;
use font_manager::FontManager;
use piston_window;
use piston_window::*;
use std::collections::HashMap;
use input;
use vector2::Vector2;
use game_state_utils;
use std::rc::Rc;
use colors;
use ui_bundle::UiBundle;
use conrod;
use conrod::Colorable;
use conrod::Sizeable;
use conrod::Widget;
use conrod::Positionable;
use conrod::image::Id;
use fps_counter::FpsCounter;

pub struct MenuScreen<'a> {
    pub world_list: Rc<Vec<&'a str>>,
    pub selected_world_index: usize,
    pub fps_counter: FpsCounter,
    pub image_map: conrod::image::Map<G2dTexture>,
    pub logo_image_id: Id, // todo: remove
}

impl<'a> GameState for MenuScreen<'a> {
    fn render(
        &mut self, 
        c: Context, 
        mut gl: &mut G2d,
        mut font_manager: &mut FontManager, 
        window_width: f64, 
        window_height: f64,
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

        conrod::widget::Canvas::new().pad(30.0).color(conrod::color::TRANSPARENT).scroll_kids_vertically().set(ui_bundle.ids.canvas, &mut ui_cell);
        conrod::widget::Text::new("WELCOME TO GUNGUN WARRIORS").font_size(36).color(conrod::color::WHITE).mid_top_of(ui_bundle.ids.canvas).set(ui_bundle.ids.title, &mut ui_cell);
        
        let mut id_widget_above = ui_bundle.ids.title;
        for i in 0..self.world_list.len() {
            let mut color = conrod::color::WHITE;
            if i == self.selected_world_index {
                color = conrod::color::BLUE;
            }

            conrod::widget::Text::new(self.world_list[i]).font_size(36).color(color).down_from(id_widget_above, 5.0).align_middle_x_of(ui_bundle.ids.canvas).set(ui_bundle.ids.world_list[i], &mut ui_cell);
            id_widget_above = ui_bundle.ids.world_list[i];
        }

        self.fps_counter.update_ui(&mut ui_cell, &ui_bundle.ids);
    }
}

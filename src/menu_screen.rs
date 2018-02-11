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

pub struct MenuScreen<'a> {
    pub world_list: Rc<Vec<&'a str>>,
    pub selected_world_index: usize,
    pub image_map: conrod::image::Map<G2dTexture>,
    pub logo_image_id: Id,
}

impl<'a> GameState for MenuScreen<'a> {
    fn render(
        &self, 
        c: Context, 
        mut gl: &mut G2d,
        mut font_manager: &mut FontManager, 
        window_width: f64, 
        window_height: f64,
        ui_bundle: &mut UiBundle) {
        
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

        // todo: move this into a func
        let mut text_vertex_data = Vec::new();
        let primitives = ui_bundle.conrod_ui.draw();
        // A function used for caching glyphs to the texture cache.
        let cache_queued_glyphs = |graphics: &mut G2d,
                                    cache: &mut G2dTexture,
                                    rect: conrod::text::rt::Rect<u32>,
                                    data: &[u8]|
        {
            let offset = [rect.min.x, rect.min.y];
            let size = [rect.width(), rect.height()];
            let format = piston_window::texture::Format::Rgba8;
            let encoder = &mut graphics.encoder;
            text_vertex_data.clear();
            text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
            piston_window::texture::UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                .expect("failed to update texture")
        };

        // Specify how to get the drawable texture from the image. In this case, the image
        // *is* the texture.
        fn texture_from_image<T>(img: &T) -> &T { img }

        // Draw the conrod `render::Primitives`.
        conrod::backend::piston::draw::primitives(primitives,
                                                    c,
                                                    gl,
                                                    &mut ui_bundle.text_texture_cache,
                                                    &mut ui_bundle.glyph_cache,
                                                    &self.image_map,
                                                    cache_queued_glyphs,
                                                    texture_from_image);
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
        let mut ui = ui_bundle.conrod_ui.set_widgets();
        conrod::widget::Canvas::new().pad(30.0).color(conrod::color::TRANSPARENT).scroll_kids_vertically().set(ui_bundle.ids.canvas, &mut ui);
        conrod::widget::Text::new("HELLO WORLD!!!\nHELLO WORLD!!!\nHELLO WORLD!!!HELLO WORLD!!!\nHELLO WORLD!!!HELLO WORLD!!!\nHELLO WORLD!!!HELLO WORLD!!!\n").font_size(42).color(conrod::color::WHITE).mid_top_of(ui_bundle.ids.canvas).set(ui_bundle.ids.title, &mut ui);
        conrod::widget::Image::new(self.logo_image_id).w_h(1280.0 * 0.25, 1280.0 * 0.25).down(60.0).align_middle_x_of(ui_bundle.ids.canvas).set(ui_bundle.ids.rust_logo, &mut ui);
    }
}

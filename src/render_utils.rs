use font_manager::FontManager;
use piston_window::Context;
use piston_window::G2d;
use piston_window::text;
use piston_window::Transformed;
use std::ops::DerefMut;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn draw_text_overlay(font_manager: &mut FontManager, c: &Context, gl: &mut G2d, window_width: f64, window_height: f64, string: &str) {
    let transform = c.transform.trans(window_width * 0.5, window_height * 0.5);
    let cache_rc = font_manager.get("Roboto-Regular.ttf");
    text(WHITE, 36, string, cache_rc.borrow_mut().deref_mut(), transform, gl);
}

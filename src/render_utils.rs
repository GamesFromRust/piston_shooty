use font_manager::FontManager;
use piston_window::Context;
use piston_window::G2d;
use piston_window::text; // piston2d-graphics
use piston_window::Transformed;
use piston_window::types::FontSize;
use piston_window::character::CharacterCache;
use std::ops::DerefMut;
use colors::Color;

pub fn draw_text_overlay(font_manager: &mut FontManager, c: &Context, gl: &mut G2d, window_width: f64, window_height: f64, x:f64, y:f64, string: &str, color: Color) {
    let font_size = 36;
    let cache_rc = font_manager.get("Roboto-Regular.ttf");
    let string_width = text_width(string, font_size, cache_rc.borrow_mut().deref_mut());
    let transform = c.transform.trans((window_width * x) - (string_width / 2.0), window_height * y);
    text(color, font_size, string, cache_rc.borrow_mut().deref_mut(), transform, gl);
}

fn text_width<C>(text: &str, font_size: FontSize, cache: &mut C) -> f64 where C: CharacterCache {
    let mut width = 0.0;
    for character in text.chars() {
        width += cache.character(font_size, character).width();
    }
    return width;
}

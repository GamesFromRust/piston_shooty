use conrod_core::position::Positionable;
use conrod_core::color::Colorable;
use conrod_core::color::Color;
use conrod_core::widget::Widget;
use crate::ui_widget_ids;

pub fn draw_text_overlay(text: &str, ui_cell: &mut conrod_core::UiCell, ids: &ui_widget_ids::Ids, color: Color, font_size: u32) {
    conrod_core::widget::Text::new(text)
        .font_size(font_size)
        .color(color)
        .middle_of(ids.canvas)
        .set(ids.title, ui_cell);
}

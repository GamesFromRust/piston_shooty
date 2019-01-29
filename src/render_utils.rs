use conrod_core::position::Positionable;
use conrod_core::color::Colorable;
use conrod_core::widget::Widget;
use crate::ui_widget_ids;

pub fn draw_text_overlay(text: &str, ui_cell: &mut conrod_core::UiCell, ids: &ui_widget_ids::Ids) {
    conrod_core::widget::Text::new(text)
        .font_size(36)
        .color(conrod_core::color::WHITE)
        .middle_of(ids.canvas)
        .set(ids.title, ui_cell);
}

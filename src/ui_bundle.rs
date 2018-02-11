use conrod;
use piston_window::G2dTexture;
use ui_widget_ids::Ids;

pub struct UiBundle<'a> {
    pub conrod_ui: conrod::Ui,
    pub glyph_cache: conrod::text::GlyphCache<'a>, // todo: make it green development at work! (why is 'a required halp)
    pub text_texture_cache: G2dTexture,
    pub ids: Ids,
}

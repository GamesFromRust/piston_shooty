use conrod;
use piston_window::G2dTexture;

pub struct UiBundle<'a> {
    pub conrod_ui: conrod::Ui,
    pub glyph_cache: conrod::text::GlyphCache<'a>, // todo: make it green development at work! (why is 'a required halp)
    pub text_texture_cache: G2dTexture,
    pub image_map: conrod::image::Map<G2dTexture>,
}
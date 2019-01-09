use conrod;
use piston_window::G2dTexture;
use crate::ui_widget_ids::Ids;
use piston_window;
use piston_window::*;

pub struct UiBundle<'a> {
    pub conrod_ui: conrod::Ui,
    pub glyph_cache: conrod::text::GlyphCache<'a>, // todo: make it green development at work! (why is 'a required halp)
    pub text_texture_cache: G2dTexture,
    pub ids: Ids,
}

impl<'a> UiBundle<'a> {
    pub fn render_ui(&mut self, 
        c: Context, 
        gl: &mut G2d,
        image_map: &conrod::image::Map<G2dTexture>) {
            
        let mut text_vertex_data = Vec::new();
        let primitives = self.conrod_ui.draw();

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
        conrod::backend::piston::draw::primitives(
            primitives,
            c,
            gl,
            &mut self.text_texture_cache,
            &mut self.glyph_cache,
            image_map,
            cache_queued_glyphs,
            texture_from_image);
    }
}

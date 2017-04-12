extern crate gfx_device_gl;
extern crate find_folder;
extern crate std;
extern crate piston_window;

use piston_window::*;

pub struct AssetLoader {
    pub assets_path: std::path::PathBuf,
    pub factory: gfx_device_gl::Factory,
}

impl AssetLoader {
    pub fn load_texture(&self,
                        relative_path: &str)
                        -> G2dTexture {
        Texture::from_path(&mut self.factory.clone(),
                           self.assets_path.join(relative_path),
                           Flip::None,
                           &TextureSettings::new())
            .unwrap()
    }

    pub fn load_font(&self, relative_path: &str) -> Glyphs  {
        Glyphs::new(self.assets_path.join(relative_path), self.factory.clone()).unwrap()
    }
}

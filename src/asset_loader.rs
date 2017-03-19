extern crate gfx_device_gl;
extern crate find_folder;
extern crate std;
extern crate piston_window;

use piston_window::*;

pub struct AssetLoader {}

impl AssetLoader {
    pub fn load_texture(&self,
                        relative_path: &str,
                        window_factory: &mut gfx_device_gl::Factory,
                        assets: &std::path::PathBuf)
                        -> G2dTexture {
        Texture::from_path(window_factory,
                           assets.join(relative_path),
                           Flip::None,
                           &TextureSettings::new())
            .unwrap()
    }
}

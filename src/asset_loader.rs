use ears::*;
use gfx_device_gl;
use piston_window::*;
use std;

pub struct AssetLoader {
    pub assets_path: std::path::PathBuf,
    pub factory: gfx_device_gl::Factory,
}

impl AssetLoader {
    pub fn load_texture(&self, relative_path: &str) -> G2dTexture {
        Texture::from_path(&mut self.factory.clone(), self.assets_path.join(relative_path), Flip::None, &TextureSettings::new()).unwrap()
    }

    pub fn load_sound(&self, relative_path: &str) -> Sound {
        Sound::new(self.assets_path.join(relative_path).to_str().unwrap()).unwrap()
    }
}

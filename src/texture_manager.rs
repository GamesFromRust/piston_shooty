use crate::asset_loader::AssetLoader;
use piston_window::*;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct TextureManager {
    pub asset_loader: Rc<AssetLoader>,
    pub textures_by_filename: HashMap<&'static str, Rc<G2dTexture>>,
}

impl TextureManager {
    pub fn get(&mut self, texture_name: &'static str) -> Rc<G2dTexture> {
        let asset_loader = &self.asset_loader;
        self.textures_by_filename.entry(texture_name).or_insert_with(|| Rc::new(asset_loader.deref().load_texture(texture_name))).clone()
    }
}

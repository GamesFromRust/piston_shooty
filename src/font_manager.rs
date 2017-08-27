use piston_window::*;
use asset_loader::AssetLoader;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

pub struct FontManager {
    pub asset_loader: Rc<AssetLoader>,
    pub fonts_by_filename: HashMap<&'static str, Rc<RefCell<Glyphs>>>
}

impl FontManager {
    pub fn get(&mut self, font_name: &'static str) -> Rc<RefCell<Glyphs>> {
        let asset_loader = &self.asset_loader;
        self.fonts_by_filename.entry(font_name).or_insert_with(|| {
            Rc::new(RefCell::new(asset_loader.deref().load_font(font_name)))
        }).clone()
    }
}
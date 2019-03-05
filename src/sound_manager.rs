use crate::asset_loader::AssetLoader;
use ears::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct SoundManager {
    pub asset_loader: Rc<AssetLoader>,
    pub sounds_by_filename: HashMap<&'static str, Rc<RefCell<Sound>>>,
}

impl SoundManager {
    pub fn get(&mut self, sound_name: &'static str) -> Rc<RefCell<Sound>> {
        let asset_loader = &self.asset_loader;
        self.sounds_by_filename.entry(sound_name).or_insert_with(|| Rc::new(RefCell::new(asset_loader.deref().load_sound(sound_name)))).clone()
    }
}

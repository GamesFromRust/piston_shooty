extern crate std;

use ears::*;
use asset_loader::AssetLoader;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

pub struct SoundManager {
    pub asset_loader: Rc<AssetLoader>,
    pub sounds_by_filename: HashMap<&'static str, Rc<RefCell<Sound>>>
}

impl SoundManager {
    pub fn get(&mut self, sound_name: &'static str) -> Rc<RefCell<Sound>> {
        let asset_loader = &self.asset_loader;
        self.sounds_by_filename.entry(sound_name).or_insert_with(|| {
            Rc::new(RefCell::new(asset_loader.deref().load_sound(sound_name)))
        }).clone()
    }
}
struct OriginGun {
    pub gun_texture: Rc<G2dTexture>,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub gun_strategy: GunStrategy,
    pub shots_taken: usize,
}

impl OriginGun {
    
}
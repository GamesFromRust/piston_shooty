use std::cell::RefCell;
use std::rc::Rc;

use crate::bullet::Bullet;
use crate::gun::Gun;
use crate::gun_behavior::GunBehavior;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use piston_window::G2dTexture;
use ears::Sound;

pub trait GunConcept {
    fn gun_texture(&self) -> &Rc<G2dTexture>;
    fn gun_image_id(&self) -> conrod_core::image::Id;
    fn selected_gun_texture(&self) -> &Rc<G2dTexture>;
    fn selected_gun_image_id(&self) -> conrod_core::image::Id;
    fn gun_sound(&self) -> &Rc<RefCell<Sound>>;
    fn bullet_sound(&self) -> &Rc<RefCell<Sound>>;
    fn gun_behavior(&self) -> &GunBehavior;
    fn guns(&self) -> &Vec<Rc<RefCell<Gun>>>;
    fn has_shot_bullet(&self) -> bool;
    fn is_selected(&self) -> bool;

    fn shots_taken(&self) -> usize;
    fn bullet_image_id(&self) -> conrod_core::image::Id;
    fn bullet_texture(&self) -> &Rc<G2dTexture>;

    fn has_guns_in_play(&self) -> bool;
    fn has_gun_depth(&self) -> bool;
    fn get_gun_depth(&self) -> usize;
    fn new_gun_behavior(&self) -> Box<GunBehavior>;
    fn set_selected(&mut self, selected: bool);
    fn update(&mut self);
    fn can_shoot_bullet(&self) -> bool;
    fn can_shoot_gun(&self) -> bool;
    fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq>;
    fn shoot_gun_from_player(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<Rc<RefCell<Gun>>>;
    // TODO: DUPLICATES world_requests_for_bullet
    fn world_requests_for_guns(&self, guns: Vec<Rc<RefCell<Gun>>>) -> Vec<WorldReq>;
    fn world_requests_for_gun(&self, gun: Rc<RefCell<Gun>>, world_reqs: &mut Vec<WorldReq>);
    fn shoot_bullets(&mut self) -> Vec<WorldReq>;
    // TODO: DUPLICATES world_requests_for_gun
    fn world_requests_for_bullet(&self, bullet: Rc<RefCell<Bullet>>) -> Vec<WorldReq>;
}

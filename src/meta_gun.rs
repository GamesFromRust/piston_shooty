use std::cell::RefCell;
use std::rc::Rc;

use ears::AudioController;
use ears::Sound;
use piston_window::G2dTexture;

use crate::bullet::Bullet;
use crate::game_object::GameObject;
use crate::gun::Gun;
use crate::gun_strategy::GunStrategy;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use crate::world::WorldRequestType;

pub struct MetaGun {
    pub gun_texture: Rc<G2dTexture>,
    pub gun_image_id: conrod_core::image::Id,
    pub selected_gun_texture: Rc<G2dTexture>,
    pub selected_gun_image_id: conrod_core::image::Id,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub bullet_texture: Rc<G2dTexture>,
    pub bullet_image_id: conrod_core::image::Id,
    pub bullet_sound: Rc<RefCell<Sound>>,
    pub gun_strategy: Box<GunStrategy>,
    pub shots_taken: usize, // drinks all around https://www.youtube.com/watch?v=XNtTEibFvlQ
    pub has_shot_bullet: bool,
    pub is_selected: bool,
}

impl MetaGun {
    pub fn has_guns_in_play(&self) -> bool {
        !self.guns.is_empty()
    }

    pub fn has_gun_depth(&self) -> bool {
        self.gun_strategy.has_gun_depth()
    }

    pub fn get_gun_depth(&self) -> usize {
        self.gun_strategy.get_gun_depth()
    }

    pub fn new_gun_strategy(&self) -> Box<GunStrategy> {
        self.gun_strategy.new_gun_strategy()
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
        if let Some(last_gun) = self.guns.last() {
            last_gun.borrow_mut().is_selected = selected;
        }
    }

    pub fn update(&mut self) {
        self.guns.retain(|ref gun| !gun.borrow().get_should_delete());
        if let Some(last_gun) = self.guns.last() {
            last_gun.borrow_mut().is_selected = true;
        }
    }

    pub fn can_shoot_bullet(&self) -> bool {
        if self.has_shot_bullet {
            return false;
        }

        if self.guns.is_empty() {
            return false;
        }

        true
    }

    fn can_shoot_gun(&self) -> bool {
        if self.has_shot_bullet {
            return false;
        }

        if self.has_gun_depth() && self.shots_taken >= self.get_gun_depth() {
            return false;
        }

        true
    }

    pub fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq> {
        self.gun_strategy.shoot_gun(&player_pos, player_rot, mouse_pos)
    }

    pub fn shoot_bullets(&mut self) -> Vec<WorldReq> {
        if !self.can_shoot_bullet() {
            return Vec::new();
        }

        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for gun in &self.guns {
            let bullet = Rc::new(RefCell::new(gun.borrow_mut().shoot_bullet(&self.bullet_texture)));
            self.bullet_sound.borrow_mut().play();
            world_reqs.append(&mut self.world_requests_for_bullet(bullet));
        }

        self.has_shot_bullet = true;

        world_reqs
    }

    // TODO: DUPLICATES world_requests_for_gun
    fn world_requests_for_bullet(&self, bullet: Rc<RefCell<Bullet>>) -> Vec<WorldReq> {
        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting

        let mut world_reqs: Vec<WorldReq> = vec![];

        let world_req: WorldReq = WorldReq {
            renderable: Some(bullet.clone()),
            updatable: None,
            collidable: Some(bullet.clone()),
            req_type: WorldRequestType::AddDynamicRenderable,
        };
        world_reqs.push(world_req);
        let world_req: WorldReq = WorldReq {
            renderable: None,
            updatable: Some(bullet.clone()),
            collidable: None,
            req_type: WorldRequestType::AddUpdatable,
        };
        world_reqs.push(world_req);
        world_reqs
    }
}

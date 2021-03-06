//use std::cell::RefCell;
//use std::rc::Rc;
//
//use ears::AudioController;
//use ears::Sound;
//use piston_window::G2dTexture;
//use piston_window::ImageSize;
//
//use crate::bullet::Bullet;
//use crate::collidable_object::CollidableObject;
//use crate::game_object::GameObject;
//use crate::gun::Gun;
//use crate::gun::GUN_SCALE;
//use crate::gun::PROJECTILE_VELOCITY_MAGNITUDE;
//use crate::gun_strategy::GunStrategy;
//use crate::object_type::ObjectType;
//use crate::renderable_object::RenderableObject;
//use crate::vector2::Vector2;
//use crate::world::WorldReq;
//use crate::world::WorldRequestType;
//
//pub struct GunConceptUtils {
//    pub gun_texture: Rc<G2dTexture>,
//    pub gun_image_id: conrod_core::image::Id,
//    pub selected_gun_texture: Rc<G2dTexture>,
//    pub selected_gun_image_id: conrod_core::image::Id,
//    pub gun_sound: Rc<RefCell<Sound>>,
//    pub bullet_texture: Rc<G2dTexture>,
//    pub bullet_image_id: conrod_core::image::Id,
//    pub bullet_sound: Rc<RefCell<Sound>>,
//    pub gun_strategy: Box<GunStrategy>,
//    pub shots_taken: usize,
//    pub guns: Vec<Rc<RefCell<Gun>>>,
//    pub has_shot_bullet: bool,
//    pub is_selected: bool,
//}
//
//impl GunConceptUtils {
//    pub fn has_guns_in_play(&self) -> bool {
//        !self.guns.is_empty()
//    }
//
//    pub fn has_gun_depth(&self) -> bool {
//        self.gun_strategy.has_gun_depth()
//    }
//
//    pub fn get_gun_depth(&self) -> usize {
//        self.gun_strategy.get_gun_depth()
//    }
//
//    pub fn new_gun_strategy(&self) -> Box<GunStrategy> {
//        self.gun_strategy.new_gun_strategy()
//    }
//
//    pub fn set_selected(&mut self, selected: bool) {
//        self.is_selected = selected;
//        if let Some(last_gun) = self.guns.last() {
//            last_gun.borrow_mut().is_selected = selected;
//        }
//    }
//
//    pub fn update(&mut self) {
//        self.guns.retain(|ref gun| !gun.borrow().get_should_delete());
//        if let Some(last_gun) = self.guns.last() {
//            last_gun.borrow_mut().is_selected = true;
//        }
//    }
//
//    pub fn can_shoot_bullet(&self) -> bool {
//        if self.has_shot_bullet {
//            return false;
//        }
//
//        if self.guns.is_empty() {
//            return false;
//        }
//
//        true
//    }
//
//    pub fn can_shoot_gun(&self) -> bool {
//        if self.has_shot_bullet {
//            return false;
//        }
//
//        if self.has_gun_depth() && self.shots_taken >= self.get_gun_depth() {
//            return false;
//        }
//
//        true
//    }
//
//    pub fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq> {
//        if !self.can_shoot_gun() {
//            return Vec::new();
//        }
//
//        let new_guns = if self.guns.is_empty() {
//            self.shoot_gun_from_player(player_pos, player_rot, mouse_pos)
//        } else if self.gun_strategy.get_object_type() == ObjectType::ShotGun {
//            let mut shot_guns: Vec<Rc<RefCell<Gun>>> = vec![]; // I see what you did there.
//            let deepest_gun_depth = if let Some(last_gun) = self.guns.last() {
//                last_gun.borrow().depth
//            } else {
//                0
//            };
//            for gun in self.guns.iter().rev() {
//                if gun.borrow().depth != deepest_gun_depth {
//                    break;
//                }
//                shot_guns.append(&mut gun.borrow().shoot_gun());
//            }
//            shot_guns
//        } else {
//            let gun = self.guns.last().unwrap();
//            gun.borrow_mut().is_selected = false;
//            gun.borrow().shoot_gun()
//        };
//
//        self.guns.append(&mut new_guns.clone());
//        self.shots_taken += 1;
//        self.world_requests_for_guns(new_guns)
//    }
//
//    fn shoot_gun_from_player(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<Rc<RefCell<Gun>>> {
//        let velocity = (*mouse_pos - *player_pos).normalized() * PROJECTILE_VELOCITY_MAGNITUDE;
//
//        let gun = Gun {
//            position: *player_pos,
//            rotation: player_rot,
//            scale: GUN_SCALE,
//            renderable_object: RenderableObject {
//                texture: self.gun_texture.clone(),
//            },
//            selected_renderable_object: RenderableObject {
//                texture: self.selected_gun_texture.clone(),
//            },
//            velocity,
//            collidable_object: CollidableObject {
//                width: f64::from(self.gun_texture.get_size().0),
//                height: f64::from(self.gun_texture.get_size().1),
//            },
//            gun_sound: self.gun_sound.clone(),
//            gun_texture: self.gun_texture.clone(),
//            selected_gun_texture: self.selected_gun_texture.clone(),
//            gun_strategy: self.new_gun_strategy(),
//            is_selected: true,
//            depth: 0,
//        };
//
//        self.gun_sound.borrow_mut().play();
//
//        vec![Rc::new(RefCell::new(gun))]
//    }
//
//    // TODO: DUPLICATES world_requests_for_bullet
//    fn world_requests_for_guns(&self, guns: Vec<Rc<RefCell<Gun>>>) -> Vec<WorldReq> {
//        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
//
//        let mut world_reqs: Vec<WorldReq> = vec![];
//
//        for gun in guns {
//            self.world_requests_for_gun(gun, &mut world_reqs);
//        }
//
//        world_reqs
//    }
//
//    fn world_requests_for_gun(&self, gun: Rc<RefCell<Gun>>, world_reqs: &mut Vec<WorldReq>) {
//        let world_req: WorldReq = WorldReq {
//            renderable: Some(gun.clone()),
//            updatable: None,
//            collidable: Some(gun.clone()),
//            req_type: WorldRequestType::AddDynamicRenderable,
//        };
//        world_reqs.push(world_req);
//        let world_req: WorldReq = WorldReq {
//            renderable: None,
//            updatable: Some(gun.clone()),
//            collidable: None,
//            req_type: WorldRequestType::AddUpdatable,
//        };
//        world_reqs.push(world_req);
//    }
//
//    pub fn shoot_bullets(&mut self) -> Vec<WorldReq> {
//        if !self.can_shoot_bullet() {
//            return Vec::new();
//        }
//
//        let mut world_reqs: Vec<WorldReq> = Vec::new();
//
//        for gun in &self.guns {
//            let bullet = Rc::new(RefCell::new(gun.borrow_mut().shoot_bullet(&self.bullet_texture)));
//            self.bullet_sound.borrow_mut().play();
//            world_reqs.append(&mut self.world_requests_for_bullet(bullet));
//        }
//
//        self.has_shot_bullet = true;
//
//        world_reqs
//    }
//
//    // TODO: DUPLICATES world_requests_for_gun
//    fn world_requests_for_bullet(&self, bullet: Rc<RefCell<Bullet>>) -> Vec<WorldReq> {
//        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
//
//        let mut world_reqs: Vec<WorldReq> = vec![];
//
//        let world_req: WorldReq = WorldReq {
//            renderable: Some(bullet.clone()),
//            updatable: None,
//            collidable: Some(bullet.clone()),
//            req_type: WorldRequestType::AddDynamicRenderable,
//        };
//        world_reqs.push(world_req);
//        let world_req: WorldReq = WorldReq {
//            renderable: None,
//            updatable: Some(bullet.clone()),
//            collidable: None,
//            req_type: WorldRequestType::AddUpdatable,
//        };
//        world_reqs.push(world_req);
//        world_reqs
//    }
//}

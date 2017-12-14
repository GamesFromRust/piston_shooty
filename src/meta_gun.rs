use gun_strategy::GunStrategy;
use std::rc::Rc;
use std::cell::RefCell;
use ears::Sound;
use piston_window::G2dTexture;
use gun::PROJECTILE_VELOCITY_MAGNITUDE;
use vector2::Vector2;
use world::WorldRequestType;
use world::WorldReq;
use bullet::Bullet;
use gun::Gun;
use renderable_object::RenderableObject;
use collidable_object::CollidableObject;
use gun::GUN_SCALE;
use ears::AudioController;
use piston_window::ImageSize;
use game_object::GameObject;

pub struct MetaGun {
    pub gun_texture: Rc<G2dTexture>,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub bullet_texture: Rc<G2dTexture>,
    pub bullet_sound: Rc<RefCell<Sound>>,
    pub gun_strategy: Box<GunStrategy>,
    pub shots_taken: usize, // drinks all around https://www.youtube.com/watch?v=XNtTEibFvlQ
    pub guns: Vec<Rc<RefCell<Gun>>>,
    pub has_shot_bullet: bool,
}

impl MetaGun {
    pub fn has_gun_depth(&self) -> bool {
        self.gun_strategy.has_gun_depth()
    }

    pub fn get_gun_depth(&self) -> usize {
        self.gun_strategy.get_gun_depth()
    }

    pub fn new_gun_strategy(&self) -> Box<GunStrategy> {
        self.gun_strategy.new_gun_strategy()
    }

    pub fn update(&mut self) {
        self.guns.retain(|ref gun| {
            !gun.borrow().get_should_delete()
        });
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

    pub fn can_shoot_gun(&self) -> bool {
        if self.has_shot_bullet {
            return false;
        }

        if self.has_gun_depth() && self.shots_taken >= self.get_gun_depth() {
            return false;
        }

        return true;
    }

    pub fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq> {
        if !self.can_shoot_gun() {
            return Vec::new();
        }

        let mut new_gun = self.shoot_gun_from_player(player_pos, player_rot, mouse_pos);
        if let Some(gun) = self.guns.last() {
             new_gun = gun.borrow().shoot_gun();
        }

        self.guns.push(new_gun.clone());
        self.shots_taken += 1;
        return self.world_requests_for_gun(new_gun);
    }

    fn shoot_gun_from_player(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Rc<RefCell<Gun>> {
        let velocity =(*mouse_pos - *player_pos).normalized() * PROJECTILE_VELOCITY_MAGNITUDE;

        let gun = Gun {
            position: *player_pos,
            rotation: player_rot,
            scale: GUN_SCALE,
            renderable_object: RenderableObject {
                texture: self.gun_texture.clone(),
            },
            velocity: velocity,
            collidable_object: CollidableObject {
                width: self.gun_texture.get_size().0 as f64,
                height: self.gun_texture.get_size().1 as f64,
            },
            gun_sound: self.gun_sound.clone(),
            gun_texture: self.gun_texture.clone(),
            gun_strategy: self.new_gun_strategy()
        };

        self.gun_sound.borrow_mut().play();

        Rc::new(RefCell::new(gun))
    }

    // TODO: DUPLICATES world_requests_for_bullet
    fn world_requests_for_gun(&self, gun: Rc<RefCell<Gun>>) -> Vec<WorldReq> {
        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
        
        let mut world_reqs: Vec<WorldReq> = vec![];

        let world_req: WorldReq = WorldReq {
            renderable: Some(gun.clone()),
            updatable: None,
            collidable: Some(gun.clone()),
            req_type: WorldRequestType::AddDynamicRenderable,
        };
        world_reqs.push(world_req);
        let world_req: WorldReq = WorldReq {
            renderable: None,
            updatable: Some(gun.clone()),
            collidable: None,
            req_type: WorldRequestType::AddUpdatable,
        };
        world_reqs.push(world_req);
        world_reqs
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
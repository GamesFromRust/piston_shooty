use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;
use updatable::Updatable;
use vector2::Vector2;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq;
use std::rc::Rc;
use std::cell::RefCell;
use collidable::Collidable;
use bullet::Bullet;
use piston_window::G2dTexture;
use collidable_object::CollidableObject;
use game_object::GameObject;
use piston_window::ImageSize;
use ears::*;
use gun_strategy::GunStrategy;

pub const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
pub const GUN_SCALE: f64 = 0.5;

pub const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
pub const BULLET_SCALE:f64 = 0.03125;
pub const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;

pub struct Gun {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub collidable_object: CollidableObject,
    pub velocity: Vector2,
    pub gun_texture: Rc<G2dTexture>,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub gun_strategy: Box<GunStrategy>
}

impl GameObject for Gun {
    fn get_position(&self) -> &Vector2 {
        &self.position
    }

    fn get_rotation(&self) -> f64 {
        self.rotation
    }
    
    fn get_scale(&self) -> f64 {
        self.scale
    }
    
    fn get_should_delete(&self) -> bool {
        self.gun_strategy.get_should_delete()
    }
    
    fn set_should_delete(&mut self, should_delete: bool) {
        self.gun_strategy.set_should_delete(should_delete)
    }
    
    fn get_object_type(&self) -> ObjectType {
        self.gun_strategy.get_object_type()
    }
}

impl Renderable for Gun {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
}

impl Updatable for Gun {
    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.position += self.velocity * args.dt;
        self.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        Vec::new()
    }
}

impl Collidable for Gun {
    fn get_collidable_object(&self) -> &CollidableObject {
        &self.collidable_object
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        self.gun_strategy.collide(other_object_type)
    }
}

impl Gun {
    pub fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Bullet {
        let velocity = Vector2 {
            x: self.rotation.cos(),
            y: self.rotation.sin(),
        };

        Bullet {
            position: self.position,
            rotation: self.rotation,
            scale: BULLET_SCALE,
            renderable_object: RenderableObject {
                texture: bullet_texture.clone(),
            },
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
            should_delete: false,
            collidable_object: CollidableObject {
                width: bullet_texture.get_size().0 as f64,
                height: bullet_texture.get_size().1 as f64,
            },
        }
    }

    pub fn shoot_gun(&self) -> Rc<RefCell<Gun>> {
        let rotation = self.get_rotation();

        let vel = Vector2 {
            x: rotation.cos(),
            y: rotation.sin(),
        };
        let velocity = vel * PROJECTILE_VELOCITY_MAGNITUDE;

        let position = *self.get_position() + (velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0;
        
        let gun = Gun {
            position: position,
            rotation: rotation,
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
            gun_strategy: self.gun_strategy.new_gun_strategy(),
        };

        self.gun_sound.borrow_mut().play();

        Rc::new(RefCell::new(gun))
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
}
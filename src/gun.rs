use crate::bullet::Bullet;
use crate::collidable::Collidable;
use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::gun_behavior::GunBehavior;
use crate::input;
use crate::object_type::ObjectType;
use crate::renderable::Renderable;
use crate::renderable_object::RenderableObject;
use crate::updatable::Updatable;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use ears::*;
use piston_window::G2dTexture;
use piston_window::ImageSize;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
pub const GUN_SCALE: f64 = 0.5;

pub const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
pub const BULLET_SCALE: f64 = 0.03125;
pub const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;

pub struct Gun {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub selected_renderable_object: RenderableObject,
    pub collidable_object: CollidableObject,
    pub velocity: Vector2,
    pub gun_texture: Rc<G2dTexture>,
    pub selected_gun_texture: Rc<G2dTexture>,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub gun_behavior: Box<GunBehavior>,
    pub is_selected: bool,
    pub depth: u32,
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
        self.gun_behavior.get_should_delete()
    }

    fn set_should_delete(&mut self, should_delete: bool) {
        self.gun_behavior.set_should_delete(should_delete)
    }

    fn get_object_type(&self) -> ObjectType {
        self.gun_behavior.get_object_type()
    }
}

impl Renderable for Gun {
    fn get_renderable_object(&self) -> &RenderableObject {
        if self.is_selected {
            &self.selected_renderable_object
        } else {
            &self.renderable_object
        }
    }
}

impl Updatable for Gun {
    #[allow(unused_variables)]
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: UpdateArgs) -> Vec<WorldReq> {
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
        self.gun_behavior.collide(other_object_type)
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
                width: f64::from(bullet_texture.get_size().0),
                height: f64::from(bullet_texture.get_size().1),
            },
        }
    }

    pub fn shoot_gun(&self) -> Vec<Rc<RefCell<Gun>>> {
        self.gun_behavior.shoot_gun(&self)
    }
}

use crate::collidable::Collidable;
use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::input;
use crate::object_type::ObjectType;
use crate::renderable::Renderable;
use crate::renderable_object::RenderableObject;
use crate::updatable::Updatable;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;

pub struct Bullet {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub velocity: Vector2,
    pub should_delete: bool,
    pub collidable_object: CollidableObject,
    pub is_visible: bool,
}

impl GameObject for Bullet {
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
        self.should_delete
    }

    fn set_should_delete(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Bullet
    }
}

impl Renderable for Bullet {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

impl Updatable for Bullet {
    #[allow(unused_variables)]
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: UpdateArgs) -> Vec<WorldReq> {
        self.position += self.velocity * args.dt;
        Vec::new()
    }
}

impl Collidable for Bullet {
    fn get_collidable_object(&self) -> &CollidableObject {
        &self.collidable_object
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        match other_object_type {
            ObjectType::Wall => {
                self.set_should_delete(true);
            }
            ObjectType::Enemy => {
                self.set_should_delete(true);
            }
            _ => {}
        }
    }
}

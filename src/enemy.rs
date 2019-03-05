use crate::collidable::Collidable;
use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::object_type::ObjectType;
use crate::renderable::Renderable;
use crate::renderable_object::RenderableObject;
use crate::vector2::Vector2;

pub struct Enemy {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub should_delete: bool,
    pub collidable_object: CollidableObject,
}

impl GameObject for Enemy {
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
        ObjectType::Enemy
    }
}

impl Renderable for Enemy {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
}

impl Collidable for Enemy {
    fn get_collidable_object(&self) -> &CollidableObject {
        &self.collidable_object
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        match other_object_type {
            ObjectType::Bullet => {
                self.set_should_delete(true);
            }
            ObjectType::GunAxe => {
                self.set_should_delete(true);
            }
            _ => {}
        }
    }
}

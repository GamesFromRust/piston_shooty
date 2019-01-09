use crate::object_type::ObjectType;
use crate::game_object::GameObject;
use crate::collidable_object::CollidableObject;

pub trait Collidable: GameObject {
    fn get_collidable_object(&self) -> &CollidableObject;
    fn collide(&mut self, other_object_type: ObjectType);
}
use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::object_type::ObjectType;

pub trait Collidable: GameObject {
    fn get_collidable_object(&self) -> &CollidableObject;
    fn collide(&mut self, other_object_type: ObjectType);
}

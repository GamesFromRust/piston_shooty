use object_type::ObjectType;
use game_object::GameObject;
use collidable_object::CollidableObject;

pub trait Collidable: GameObject {
    fn get_collidable_object(&self) -> &CollidableObject;
    fn collide(&self, other_object_type: ObjectType);
}
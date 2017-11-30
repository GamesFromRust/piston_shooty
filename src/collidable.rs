use object_type::ObjectType;
use game_object::GameObject;
use collidable_object::CollidableObject;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;

pub trait Collidable: GameObject {
    fn get_collidable_object(&self) -> &CollidableObject;
    fn collide(&mut self, other_object_type: ObjectType);
}
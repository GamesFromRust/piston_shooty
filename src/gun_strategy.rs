use object_type::ObjectType;
use gun::Gun;

pub trait GunStrategy {
    fn get_should_delete(&self) -> bool;
    fn set_should_delete(&mut self, should_delete: bool);
    fn collide(&mut self, other_object_type: ObjectType);
}
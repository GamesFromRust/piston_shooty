use crate::object_type::ObjectType;

pub trait GunStrategy {
    fn get_should_delete(&self) -> bool;
    fn set_should_delete(&mut self, should_delete: bool);
    fn get_object_type(&self) -> ObjectType;
    fn collide(&mut self, other_object_type: ObjectType);
    fn new_gun_strategy(&self) -> Box<GunStrategy>;
    fn has_gun_depth(&self) -> bool;
    fn get_gun_depth(&self) -> usize;
}

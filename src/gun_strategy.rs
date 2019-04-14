use crate::object_type::ObjectType;
use crate::vector2::Vector2;
use crate::world::WorldReq;

pub trait GunStrategy {
    fn get_should_delete(&self) -> bool;
    fn set_should_delete(&mut self, should_delete: bool);
    fn get_object_type(&self) -> ObjectType;
    fn collide(&mut self, other_object_type: ObjectType);
    fn new_gun_strategy(&self) -> Box<GunStrategy>;
    fn has_gun_depth(&self) -> bool;
    fn get_gun_depth(&self) -> usize;
    fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq>;
}

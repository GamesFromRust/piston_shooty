use object_type::ObjectType;
use vector2::Vector2;

pub trait GameObject {
    fn get_position(&self) -> &Vector2;
    fn get_rotation(&self) -> f64;
    fn get_scale(&self) -> f64;
    fn get_should_delete(&self) -> bool;
    fn set_should_delete(&mut self, should_delete: bool);
    fn get_object_type(&self) -> ObjectType;
}

use renderable_object::RenderableObject;
use object_type::ObjectType;

pub trait Renderable {
    fn get_renderable_object(&self) -> &RenderableObject;
    fn get_should_delete_renderable(&self) -> bool;
    fn set_should_delete_renderable(&mut self, should_delete: bool);
    fn get_object_type(&self) -> ObjectType;
}
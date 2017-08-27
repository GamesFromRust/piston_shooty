use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;

pub struct Enemy {
    pub renderable_object: RenderableObject,
    pub should_delete: bool,
}

impl Renderable for Enemy {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Enemy
    }
}
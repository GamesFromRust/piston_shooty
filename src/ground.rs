use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;

pub struct Ground {
    pub renderable_object: RenderableObject,
}

impl Renderable for Ground {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        // do nothing
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Ground
    }
}
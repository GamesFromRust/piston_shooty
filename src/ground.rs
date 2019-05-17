use crate::game_object::GameObject;
use crate::object_type::ObjectType;
use crate::renderable::Renderable;
use crate::renderable_object::RenderableObject;
use crate::vector2::Vector2;

pub struct Ground {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub is_visible: bool,
}

impl GameObject for Ground {
    fn get_position(&self) -> &Vector2 {
        &self.position
    }

    fn get_rotation(&self) -> f64 {
        self.rotation
    }

    fn get_scale(&self) -> f64 {
        self.scale
    }

    fn get_should_delete(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn set_should_delete(&mut self, should_delete: bool) {
        // do nothing
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Ground
    }
}

impl Renderable for Ground {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

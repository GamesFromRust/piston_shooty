use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;
use vector2::Vector2;
use updatable::Updatable;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq;

pub struct Bullet {
    pub renderable_object: RenderableObject,
    pub velocity: Vector2,
    pub should_delete: bool,
}

impl Renderable for Bullet {
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
        ObjectType::Bullet
    }
}

impl Updatable for Bullet {
    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.renderable_object.position += self.velocity * args.dt;
        Vec::new()
    }

    fn get_should_delete_updatable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_updatable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }
}
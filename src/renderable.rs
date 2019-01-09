use crate::renderable_object::RenderableObject;
use crate::game_object::GameObject;

pub trait Renderable: GameObject {
    fn get_renderable_object(&self) -> &RenderableObject;
}
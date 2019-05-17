use crate::game_object::GameObject;
use crate::renderable_object::RenderableObject;

pub trait Renderable: GameObject {
    fn get_renderable_object(&self) -> &RenderableObject;
    fn is_visible(&self) -> bool;
}

use renderable_object::RenderableObject;
use game_object::GameObject;

pub trait Renderable: GameObject {
    fn get_renderable_object(&self) -> &RenderableObject;
}
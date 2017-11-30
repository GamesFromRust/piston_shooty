use renderable_object::RenderableObject;
use game_object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;

pub trait Renderable: GameObject {
    fn get_renderable_object(&self) -> &RenderableObject;
}
use vector2::*;
use piston_window::*;
use std::rc::Rc;

pub struct RenderableObject {
    pub texture: Rc<G2dTexture>,
}
use vector2::*;
use piston_window::*;
use std::rc::Rc;

pub struct RenderableObject {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub texture: Rc<G2dTexture>,
}
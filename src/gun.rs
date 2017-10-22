use bullet::Bullet;
use piston_window::G2dTexture;
use std::rc::Rc;
use std::cell::RefCell;
use renderable::Renderable;
use updatable::Updatable;
use collidable::Collidable;
use world::WorldReq;

pub const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
pub const BULLET_SCALE:f64 = 0.03125;
pub const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;

pub trait Gun: Renderable + Updatable + Collidable {
    fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Bullet;
    fn shoot_gun(&self) -> Rc<RefCell<Gun>>;
}

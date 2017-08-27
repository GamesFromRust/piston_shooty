use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;
use updatable::Updatable;
use vector2::Vector2;
use bullet::Bullet;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq;
use std::rc::Rc;
use piston_window::G2dTexture;

const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
const BULLET_SCALE:f64 = 0.03125;
const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;

pub struct Gun {
    pub renderable_object: RenderableObject,
    pub velocity: Vector2,
    pub should_delete: bool,
}

impl Renderable for Gun {
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
        ObjectType::Gun
    }
}

impl Updatable for Gun {
    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.renderable_object.position += self.velocity * args.dt;
        self.renderable_object.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        Vec::new()
    }

    fn get_should_delete_updatable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_updatable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }
}

impl Gun {
    pub fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Bullet {
        let velocity = Vector2 {
            x: self.renderable_object.rotation.cos(),
            y: self.renderable_object.rotation.sin(),
        };

        Bullet {
            renderable_object: RenderableObject {
                position: self.renderable_object.position,
                texture: bullet_texture.clone(),
                rotation: self.renderable_object.rotation,
                scale: BULLET_SCALE,
            },
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
            should_delete: false,
        }
    }
}
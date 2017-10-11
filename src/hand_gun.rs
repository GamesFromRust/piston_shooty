use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;
use updatable::Updatable;
use vector2::Vector2;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq;
use std::rc::Rc;
use collidable::Collidable;
use bullet::Bullet;
use piston_window::G2dTexture;
use gun::GUN_ROTATIONAL_VELOCITY;
use gun::BULLET_VELOCITY_MAGNITUDE;
use gun::BULLET_SCALE;
use gun::Gun;
use collidable_object::CollidableObject;
use game_object::GameObject;
use piston_window::ImageSize;

pub struct HandGun {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub collidable_object: CollidableObject,
    pub velocity: Vector2,
    pub should_delete: bool,
}

impl GameObject for HandGun {
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
        self.should_delete
    }
    
    fn set_should_delete(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }
    
    fn get_object_type(&self) -> ObjectType {
        ObjectType::HandGun
    }
}

impl Renderable for HandGun {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
}

impl Updatable for HandGun {
    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.position += self.velocity * args.dt;
        self.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        Vec::new()
    }
}

impl Collidable for HandGun {
    fn get_collidable_object(&self) -> &CollidableObject {
        &self.collidable_object
    }

    fn collide(&self, other_object_type: ObjectType) {
        
    }
}

impl Gun for HandGun {
    fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Bullet {
        let velocity = Vector2 {
            x: self.rotation.cos(),
            y: self.rotation.sin(),
        };

        Bullet {
            position: self.position,
            rotation: self.rotation,
            scale: BULLET_SCALE,
            renderable_object: RenderableObject {
                texture: bullet_texture.clone(),
            },
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
            should_delete: false,
            collidable_object: CollidableObject {
                width: bullet_texture.get_size().0 as f64,
                height: bullet_texture.get_size().1 as f64,
            },
        }
    }
}
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
use std::rc::Rc;
use std::cell::RefCell;
use piston_window::G2dTexture;
use hand_gun::HandGun;
use ears::*;
use world::WorldRequestType;
use gun::Gun;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
const GUN_SCALE: f64 = 0.5;

pub struct Player {
    pub renderable_object: RenderableObject,
    pub guns: Vec<Rc<RefCell<Gun>>>,
    pub gun_texture: Rc<G2dTexture>,
    pub gun_sound: Rc<RefCell<Sound>>,
    pub bullet_texture: Rc<G2dTexture>,
    pub bullet_sound: Rc<RefCell<Sound>>,
    pub has_shot: bool,
}

impl Renderable for Player {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        // do nothing
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Player
    }
}

impl Updatable for Player {
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.guns.retain(|ref gun| {
            !gun.borrow().get_should_delete_updatable()
        });
        
        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.renderable_object.position;
        self.renderable_object.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        return self.apply_input(&key_states, &mouse_states, &mouse_pos, args.dt);
    }
    
    fn get_should_delete_updatable(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn set_should_delete_updatable(&mut self, should_delete: bool) {
        // do nothing
    }
}

impl Player {
    fn shoot_bullets(&mut self) -> Vec<WorldReq> {
        if self.has_shot {
            return Vec::new();
        }

        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for projectile in &self.guns {
            let bullet = Rc::new(RefCell::new(projectile.borrow_mut().shoot_bullet(&self.bullet_texture)));
            self.bullet_sound.borrow_mut().play();

            let world_req: WorldReq = WorldReq {
                renderable: Some(bullet.clone()),
                updatable: None,
                req_type: WorldRequestType::AddDynamicRenderable,
            };
            world_reqs.push(world_req);

            let world_req: WorldReq = WorldReq {
                renderable: None,
                updatable: Some(bullet.clone()),
                req_type: WorldRequestType::AddUpdatable,
            };
            world_reqs.push(world_req);
        }

        if !world_reqs.is_empty() {
            self.has_shot = true;
        }

        world_reqs
    }

    fn shoot_gun(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq>  {
        if self.has_shot {
            return Vec::new()
        }

        let rotation = match self.guns.last() {
            Some(gun) => gun.borrow().get_renderable_object().rotation,
            None => self.renderable_object.rotation,
        };

        let velocity = match self.guns.last() {
            Some(_) => {
                let vel = Vector2 {
                    x: rotation.cos(),
                    y: rotation.sin(),
                };
                vel * PROJECTILE_VELOCITY_MAGNITUDE
            },
            None => (*mouse_pos - self.renderable_object.position).normalized() * PROJECTILE_VELOCITY_MAGNITUDE,
        };

        let position = match self.guns.last() {
            Some(gun) => gun.borrow().get_renderable_object().position + ( velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0,
            None => self.renderable_object.position,
        };

        let projectile = HandGun {
            renderable_object: RenderableObject {
                position: position,
                texture: self.gun_texture.clone(),
                rotation: rotation,
                scale: GUN_SCALE,
            },
            velocity: velocity,
            should_delete: false,
        };

        self.gun_sound.borrow_mut().play();

        let projectile = Rc::new(RefCell::new(projectile));
        self.guns.push(projectile.clone());

        let mut world_reqs: Vec<WorldReq> = Vec::new();
        
        let world_req: WorldReq = WorldReq {
            renderable: Some(projectile.clone()),
            updatable: None,
            req_type: WorldRequestType::AddDynamicRenderable,
        };
        world_reqs.push(world_req);

        let world_req: WorldReq = WorldReq {
            renderable: None,
            updatable: Some(projectile.clone()),
            req_type: WorldRequestType::AddUpdatable,
        };
        world_reqs.push(world_req);

        world_reqs
    }

    #[allow(unused_variables)]
    fn apply_input(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, dt: f64) -> Vec<WorldReq> {
        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for (button, value) in mouse_states {
            match *button {
                MouseButton::Left => {
                    if value.pressed {
                        world_reqs.append(&mut self.shoot_gun(mouse_pos));
                    }
                },
                MouseButton::Right => {
                    if value.pressed {
                        world_reqs.append(&mut self.shoot_bullets());
                    }
                }
                // Default
                _ => {}
            }
        }

        world_reqs
    }
}
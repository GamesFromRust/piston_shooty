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
use gun::Gun;
use ears::*;
use world::WorldRequestType;
use gun::PROJECTILE_VELOCITY_MAGNITUDE;
use gun::GUN_SCALE;
use game_object::GameObject;
use collidable_object::CollidableObject;
use piston_window::ImageSize;

pub struct Player {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub guns: Vec<Rc<RefCell<Gun>>>,
    pub bullet_texture: Rc<G2dTexture>,
    pub bullet_sound: Rc<RefCell<Sound>>,
    pub has_shot_bullet: bool,
    pub gun_template: Rc<Gun>,
    pub gun_templates: Vec<Rc<Gun>>,
    pub current_gun_template_index: usize,
}

impl GameObject for Player {
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
        false
    }
    
    #[allow(unused_variables)]
    fn set_should_delete(&mut self, should_delete: bool) {
        // do nothing
    }
    
    fn get_object_type(&self) -> ObjectType {
        ObjectType::Player
    }
}

impl Renderable for Player {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
}

impl Updatable for Player {
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.guns.retain(|ref gun| {
            !gun.borrow().get_should_delete()
        });
        
        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.position;
        self.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        return self.apply_input(&key_states, &mouse_states, &mouse_pos, args.dt);
    }
}

impl Player {
    fn shoot_bullets(&mut self) -> Vec<WorldReq> {
        if self.has_shot_bullet {
            return Vec::new();
        }

        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for projectile in &self.guns {
            let bullet = Rc::new(RefCell::new(projectile.borrow_mut().shoot_bullet(&self.bullet_texture)));
            self.bullet_sound.borrow_mut().play();

            let world_req: WorldReq = WorldReq {
                renderable: Some(bullet.clone()),
                updatable: None,
                collidable: Some(bullet.clone()),
                req_type: WorldRequestType::AddDynamicRenderable,
            };
            world_reqs.push(world_req);

            let world_req: WorldReq = WorldReq {
                renderable: None,
                updatable: Some(bullet.clone()),
                collidable: None,
                req_type: WorldRequestType::AddUpdatable,
            };
            world_reqs.push(world_req);
        }

        if !world_reqs.is_empty() {
            self.has_shot_bullet = true;
        }

        world_reqs
    }

    fn shoot_gun_from_player(&mut self, mouse_pos: &Vector2) -> Rc<RefCell<Gun>> {
        let rotation = self.rotation;

        let velocity =(*mouse_pos - self.position).normalized() * PROJECTILE_VELOCITY_MAGNITUDE;

        let position = self.position;

        let projectile = Gun {
            position: position,
            rotation: rotation,
            scale: GUN_SCALE,
            renderable_object: RenderableObject {
                texture: self.gun_template.gun_texture.clone(),
            },
            velocity: velocity,
            collidable_object: CollidableObject {
                width: self.gun_template.gun_texture.get_size().0 as f64,
                height: self.gun_template.gun_texture.get_size().1 as f64,
            },
            gun_sound: self.gun_template.gun_sound.clone(),
            gun_texture: self.gun_template.gun_texture.clone(),
            gun_strategy: self.gun_template.new_gun_strategy()
        };

        self.gun_template.gun_sound.borrow_mut().play();

        Rc::new(RefCell::new(projectile))
    }

    fn world_requests_from(&self, gun: Rc<RefCell<Gun>>) -> Vec<WorldReq> {
        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
        
        let mut world_reqs: Vec<WorldReq> = vec![];

        let world_req: WorldReq = WorldReq {
            renderable: Some(gun.clone()),
            updatable: None,
            collidable: Some(gun.clone()),
            req_type: WorldRequestType::AddDynamicRenderable,
        };
        world_reqs.push(world_req);
        let world_req: WorldReq = WorldReq {
            renderable: None,
            updatable: Some(gun.clone()),
            collidable: None,
            req_type: WorldRequestType::AddUpdatable,
        };
        world_reqs.push(world_req);
        world_reqs
    }

    fn can_shoot(&self) -> bool {
        if self.has_shot_bullet {
            return false;
        }

        if self.gun_template.has_gun_depth() && self.guns.len() >= self.gun_template.get_gun_depth() {
            return false;
        }

        return true;
    }

    fn shoot_gun(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq>  {
        if !self.can_shoot() {
            return Vec::new();
        }

        let mut new_gun = self.shoot_gun_from_player(&mouse_pos);
        if let Some(gun) = self.guns.last() {
             new_gun = gun.borrow().shoot_gun();
        }

        self.guns.push(new_gun.clone());
        self.world_requests_from(new_gun)
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
                _ => {}
            }
        }

        for (key, value) in key_states {
            match *key {
                Key::Q => {
                    if value.pressed {
                        if self.current_gun_template_index == self.gun_templates.len() - 1 {
                            self.current_gun_template_index = 0;
                        } else {
                            self.current_gun_template_index += 1;
                        }
                        self.gun_template = self.gun_templates[self.current_gun_template_index].clone();
                    }
                },
                Key::E => {
                    if value.pressed {
                        if self.current_gun_template_index == 0 {
                            self.current_gun_template_index = self.gun_templates.len();
                        } else {
                            self.current_gun_template_index -= 1;
                        }
                        self.gun_template = self.gun_templates[self.current_gun_template_index].clone();
                    }
                }
                _ => {}
            }
        }

        world_reqs
    }
}
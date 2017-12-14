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
use gun::GUN_SCALE;
use game_object::GameObject;
use collidable_object::CollidableObject;
use piston_window::ImageSize;
use meta_gun::MetaGun;
use std::cell::Ref;
use std::cell::RefMut;

pub struct Player {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub gun_templates: Vec<RefCell<MetaGun>>,
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
        self.meta_gun_mut().update();

        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.position;
        self.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        return self.apply_input(&key_states, &mouse_states, &mouse_pos, args.dt);
    }
}

impl Player {
    fn meta_gun(&self) -> Ref<MetaGun> {
        return self.gun_templates[self.current_gun_template_index].borrow()
    }

    fn meta_gun_mut(&self) -> RefMut<MetaGun> {
        return self.gun_templates[self.current_gun_template_index].borrow_mut()
    }

    fn shoot_bullets(&mut self) -> Vec<WorldReq> {
        return self.meta_gun_mut().shoot_bullets();
    }
    
    pub fn can_shoot_bullet(&self) -> bool {
        for gun_template in &self.gun_templates {
            if gun_template.borrow().can_shoot_bullet() {
                return true;
            }
        }
        return false;
    }

    pub fn can_shoot_gun(&self) -> bool {
        for gun_template in &self.gun_templates {
            if gun_template.borrow().can_shoot_gun() {
                return true;
            }
        }
        return false;
    }

    fn shoot_gun(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq>  {
        return self.meta_gun_mut().shoot_gun(&self.position, self.rotation, mouse_pos);
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
                    }
                },
                Key::E => {
                    if value.pressed {
                        if self.current_gun_template_index == 0 {
                            self.current_gun_template_index = self.gun_templates.len();
                        } else {
                            self.current_gun_template_index -= 1;
                        }
                    }
                }
                _ => {}
            }
        }

        world_reqs
    }
}
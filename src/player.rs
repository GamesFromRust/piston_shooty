use crate::game_object::GameObject;
use crate::input;
use crate::gun_concept::GunConcept;
use crate::object_type::ObjectType;
use crate::renderable::Renderable;
use crate::renderable_object::RenderableObject;
use crate::updatable::Updatable;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Player {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub selected_renderable_object: RenderableObject,
    pub gun_concepts: Vec<Rc<RefCell<GunConcept>>>,
    pub current_gun_concept_index: usize,
    pub is_visible: bool,
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
        if self.gun_concepts[self.current_gun_concept_index].borrow().has_guns_in_play() {
            &self.renderable_object
        } else {
            &self.selected_renderable_object
        }
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

impl Updatable for Player {
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: UpdateArgs) -> Vec<WorldReq> {
        self.gun_concept_mut().update();

        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.position;
        self.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        self.apply_input(&key_states, &mouse_states, &mouse_pos, args.dt)
    }
}

impl Player {
    fn gun_concept_mut(&self) -> RefMut<GunConcept> {
        self.gun_concepts[self.current_gun_concept_index].borrow_mut()
    }

    fn bullet_trigger_pressed(&mut self) -> Vec<WorldReq> {
        self.gun_concept_mut().bullet_trigger_pressed()
    }

    pub fn can_shoot_bullet(&self) -> bool {
        self.gun_concepts.iter().any(|gun_concept| gun_concept.borrow().can_shoot_bullet())
    }

    pub fn can_shoot_gun(&self) -> bool {
        self.gun_concepts.iter().any(|gun_concept| gun_concept.borrow().can_shoot_gun())
    }

    fn gun_trigger_pressed(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq> {
        self.gun_concept_mut().gun_trigger_pressed(&self.position, self.rotation, mouse_pos)
    }

    fn gun_trigger_held(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq> {
        self.gun_concept_mut().gun_trigger_held(&self.position, self.rotation, mouse_pos)
    }

    #[allow(unused_variables)]
    fn apply_input(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, dt: f64) -> Vec<WorldReq> {
        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for (button, value) in mouse_states {
            match *button {
                MouseButton::Left => {
                    if value.pressed {
                        world_reqs.append(&mut self.gun_trigger_pressed(mouse_pos));
                    }
                    if value.held {
                        world_reqs.append(&mut self.gun_trigger_held(mouse_pos));
                    }
                }
                MouseButton::Right => {
                    if value.pressed {
                        world_reqs.append(&mut self.bullet_trigger_pressed());
                    }
                }
                _ => {}
            }
        }

        for (key, value) in key_states {
            match *key {
                Key::Q => {
                    if value.pressed {
                        self.gun_concepts[self.current_gun_concept_index].borrow_mut().set_selected(false);

                        self.current_gun_concept_index += 1;
                        self.current_gun_concept_index %= self.gun_concepts.len();

                        self.gun_concepts[self.current_gun_concept_index].borrow_mut().set_selected(true);
                    }
                }
                Key::E => {
                    if value.pressed {
                        self.gun_concepts[self.current_gun_concept_index].borrow_mut().set_selected(false);

                        if self.current_gun_concept_index == 0 {
                            self.current_gun_concept_index = self.gun_concepts.len() - 1;
                        } else {
                            self.current_gun_concept_index -= 1;
                        }

                        self.gun_concepts[self.current_gun_concept_index].borrow_mut().set_selected(true);
                    }
                }
                _ => {}
            }
        }

        world_reqs
    }
}

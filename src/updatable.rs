use vector2::Vector2;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq; // circular dependency?
use game_object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;

pub trait Updatable: GameObject {
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> Vec<WorldReq>;
}
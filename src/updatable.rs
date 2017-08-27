use vector2::Vector2;
use input;
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;
use world::WorldReq; // circular dependency?

pub trait Updatable {
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> Vec<WorldReq>;
    // TODO: Deletable?
    fn get_should_delete_updatable(&self) -> bool;
    fn set_should_delete_updatable(&mut self, should_delete: bool);
}
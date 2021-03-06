use crate::game_object::GameObject;
use crate::input;
use crate::vector2::Vector2;
use crate::world::WorldReq; // circular dependency?
use piston_window::Key;
use piston_window::MouseButton;
use piston_window::UpdateArgs;
use std::collections::HashMap;

pub trait Updatable: GameObject {
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: UpdateArgs) -> Vec<WorldReq>;
}

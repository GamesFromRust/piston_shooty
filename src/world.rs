use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use std::rc::Rc;
use std::cell::RefCell;
use ncollide_geometry;
use ncollide_geometry::shape::Cuboid2;
use ncollide_geometry::bounding_volume;
use ncollide_geometry::bounding_volume::BoundingVolume;
use std::sync::mpsc::Receiver;
use input;
use renderable_object::RenderableObject;
use nalgebra;
use renderable::Renderable;
use object_type::ObjectType;
use updatable::Updatable;
use player::Player;
use std::cmp;
use font_manager::FontManager;
use std::ops::Deref;
use game_state::GameState;
use game_state::UpdateResult;
use render_utils;
use game_state_utils;

const ENEMY_LAYER: usize = 1;
const PROJECTILE_LAYER: usize = 2;

pub struct GameEndedState {
    pub game_ended: bool,
    pub won: bool,
}

pub enum WorldRequestType {
    AddUpdatable,
    AddDynamicRenderable,
}

pub struct WorldReq {
    pub renderable: Option<Rc<RefCell<Renderable>>>,
    pub updatable: Option<Rc<RefCell<Updatable>>>,
    pub req_type: WorldRequestType,
}

// TODO: Add self/guns/bullets to here.
pub struct World {
    pub static_renderables: Vec<Vec<Rc<Renderable>>>,
    pub dynamic_renderables: Vec<Vec<Rc<RefCell<Renderable>>>>,
    pub updatables: Vec<Rc<RefCell<Updatable>>>,
    pub game_ended_state: GameEndedState,
    pub player: Rc<RefCell<Player>>,
    pub receiver: Receiver<u64>,
    pub should_display_level_name: bool,
    pub name: String,
}

impl World {
    pub fn add_static_renderable_at_layer(&mut self, renderable: Rc<Renderable>, layer: usize) {
        while self.static_renderables.len() <= layer {
            self.static_renderables.push(Vec::new());
        }
        self.static_renderables[layer].push(renderable);
    }

    pub fn add_dynamic_renderable_at_layer(&mut self, renderable: Rc<RefCell<Renderable>>, layer: usize) {
        while self.dynamic_renderables.len() <= layer {
            self.dynamic_renderables.push(Vec::new());
        }
        self.dynamic_renderables[layer].push(renderable);
    }

    pub fn add_updatable(&mut self, updatable: Rc<RefCell<Updatable>>) {
        self.updatables.push(updatable);
    }

    fn is_victorious(&self) -> bool {
        for renderable in &self.dynamic_renderables[ENEMY_LAYER] {
            if renderable.borrow().get_object_type() == ObjectType::Enemy {
                return false;
            }
        }
        
        true
    }

    fn was_defeated(&self) -> bool {
        if !self.player.borrow().has_shot {
            return false;
        }
        
        for renderable_layer in &self.dynamic_renderables {
            for renderable in renderable_layer {
                if renderable.borrow().get_object_type() == ObjectType::Bullet {
                    return false;
                }
            }
        }

        true
    }

    fn update_game_running(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> UpdateResult {
        let _ = self.receiver.try_recv().map(|_| self.should_display_level_name = false);

        if self.is_victorious() {
            self.game_ended_state = GameEndedState { game_ended: true, won: true };
            return UpdateResult::Running;
        }

        if self.was_defeated() {
            self.game_ended_state = GameEndedState { game_ended: true, won: false };
            return UpdateResult::Running;
        }

        for renderable_layer in &self.dynamic_renderables {
            for renderable in renderable_layer {
                for renderable_layer2 in &self.static_renderables {
                    for renderable2 in renderable_layer2 {
                        if renderable.borrow().get_object_type() == ObjectType::Gun && renderable2.get_object_type() == ObjectType::Wall {
                            let renderable1_aabb_cuboid2 = create_aabb_cuboid2(&renderable.borrow().get_renderable_object());
                            let renderable2_aabb_cuboid2 = create_aabb_cuboid2(&renderable2.get_renderable_object());
                            
                            if renderable1_aabb_cuboid2.intersects(&renderable2_aabb_cuboid2) {
                                renderable.borrow_mut().set_should_delete_renderable(true);
                            }
                        }

                        if renderable.borrow().get_object_type() == ObjectType::Bullet && renderable2.get_object_type() == ObjectType::Wall {
                            let renderable1_aabb_cuboid2 = create_aabb_cuboid2(&renderable.borrow().get_renderable_object());
                            let renderable2_aabb_cuboid2 = create_aabb_cuboid2(&renderable2.get_renderable_object());
                            
                            if renderable1_aabb_cuboid2.intersects(&renderable2_aabb_cuboid2) {
                                renderable.borrow_mut().set_should_delete_renderable(true);
                            }
                        }
                    }
                }

                for renderable_layer2 in &self.dynamic_renderables {
                    for renderable2 in renderable_layer2 {
                        if renderable.borrow().get_object_type() == ObjectType::Bullet && renderable2.borrow().get_object_type() == ObjectType::Enemy {
                            let renderable1_aabb_cuboid2 = create_aabb_cuboid2(&renderable.borrow().get_renderable_object());
                            let renderable2_aabb_cuboid2 = create_aabb_cuboid2(&renderable2.borrow().get_renderable_object());
                            
                            if renderable1_aabb_cuboid2.intersects(&renderable2_aabb_cuboid2) {
                                renderable.borrow_mut().set_should_delete_renderable(true);
                                renderable2.borrow_mut().set_should_delete_renderable(true);
                            }
                        }
                    }
                }
            }
        }

        for renderable_layer in &mut self.dynamic_renderables {
            renderable_layer.retain(|ref renderable| {
                !renderable.borrow().get_should_delete_renderable()
            });
        }

        self.updatables.retain(|ref updatable| {
            !updatable.borrow().get_should_delete_updatable()
        });

        let mut world_reqs: Vec<WorldReq> = Vec::new();
        for updatable in &self.updatables {
            let current_world_reqs = &mut updatable.borrow_mut().update(&key_states, &mouse_states, &mouse_pos, &args);
            world_reqs.append(current_world_reqs);
        }
        
        for world_req in world_reqs {
            match world_req.req_type {
                WorldRequestType::AddDynamicRenderable => {
                    assert!(world_req.renderable.is_some());
                    if let Some(renderable) = world_req.renderable {
                        self.add_dynamic_renderable_at_layer(renderable, PROJECTILE_LAYER);
                    }
                },
                WorldRequestType::AddUpdatable => {
                    assert!(world_req.updatable.is_some());
                    if let Some(updatable) = world_req.updatable {
                        self.add_updatable(updatable);
                    }
                },
            }
        }

        UpdateResult::Running
    }

    fn update_game_ended_lost(&self, mouse_states: &HashMap<MouseButton, input::ButtonState>) -> UpdateResult {
        if game_state_utils::did_click(&mouse_states) {
            return UpdateResult::Fail;
        } else {
            return UpdateResult::Running;
        }
    }

    fn update_game_ended_won(&self, mouse_states: &HashMap<MouseButton, input::ButtonState>) -> UpdateResult {
        if game_state_utils::did_click(&mouse_states) {
            return UpdateResult::Success;
        } else {
            return UpdateResult::Running;
        }
    }
}

impl GameState for World {
    fn render(&self, c: Context, mut gl: &mut G2d, mut font_manager: &mut FontManager, window_width: f64, window_height: f64) {
        let max_layers = cmp::max(self.static_renderables.len(), self.dynamic_renderables.len());
        for i in 0..max_layers {
            if i < self.static_renderables.len() {
                for renderable in &self.static_renderables[i] {
                    let renderable_object = renderable.get_renderable_object();
                    render_renderable_object(&c, &mut gl, &renderable_object);
                }
            }
            if i < self.dynamic_renderables.len() {
                for renderable in &self.dynamic_renderables[i] {
                    // TODO: Why can't we do this?
                    // let renderable_object = renderable.borrow().get_renderable_object();
                    // render_renderable_object(&c, &mut gl, &renderable_object);
                    render_renderable_object(&c, &mut gl, &renderable.borrow().get_renderable_object());
                }
            }
        }

        if self.game_ended_state.game_ended {
            if self.game_ended_state.won {
                render_utils::draw_text_overlay(&mut font_manager, &c, &mut gl, window_width, window_height, "Success! Click to continue.");
            } else {
                render_utils::draw_text_overlay(&mut font_manager, &c, &mut gl, window_width, window_height, "Defeat! Click to retry.");
            }
        } else if self.should_display_level_name {
            render_utils::draw_text_overlay(&mut font_manager, &c, &mut gl, window_width, window_height, self.name.as_str());
        }
    }

    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> UpdateResult {
        if self.game_ended_state.game_ended == false && self.game_ended_state.won == false {
            return self.update_game_running(&key_states, &mouse_states, &mouse_pos, &args);
        }

        if self.game_ended_state.game_ended == true && self.game_ended_state.won == false {
            return self.update_game_ended_lost(&mouse_states);
        }

        if self.game_ended_state.game_ended == true && self.game_ended_state.won == true {
            return self.update_game_ended_won(&mouse_states);
        }
        
        assert_eq!(false, true, "Invalid game ended state! Shouldn't have gotten here!");
        UpdateResult::Running
    }
}

fn create_aabb_cuboid2(renderable_object: &RenderableObject) -> ncollide_geometry::bounding_volume::AABB<nalgebra::PointBase<f64, nalgebra::U2, nalgebra::MatrixArray<f64, nalgebra::U2, nalgebra::U1>>> {
    let half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(
        renderable_object.texture.get_size().0 as f64 * 0.5 * renderable_object.scale,
        renderable_object.texture.get_size().1 as f64 * 0.5 * renderable_object.scale);
    let cuboid2 = Cuboid2::new(half_extents);
    let cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(renderable_object.position.x, renderable_object.position.y), renderable_object.rotation);
    let aabb_cuboid2 = bounding_volume::aabb(&cuboid2, &cuboid2_pos);
    aabb_cuboid2
}

fn render_renderable_object(c: &Context, gl: &mut G2d, renderable_object: &RenderableObject) {
    let transform = c.transform
        .trans(renderable_object.position.x, renderable_object.position.y)
        .rot_rad(renderable_object.rotation)
        .trans((renderable_object.texture.get_size().0 as f64) * -0.5 * renderable_object.scale,
                (renderable_object.texture.get_size().1 as f64) * -0.5 * renderable_object.scale)
        .scale(renderable_object.scale, renderable_object.scale);
    image(renderable_object.texture.deref(), transform, gl);
}

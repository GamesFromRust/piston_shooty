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

    pub fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) {
        let _ = self.receiver.try_recv().map(|_| self.should_display_level_name = false);

        // check for victory
        let mut no_enemies = true;
        for renderable in &self.dynamic_renderables[ENEMY_LAYER] {
            if renderable.borrow().get_object_type() == ObjectType::Enemy {
                no_enemies = false;
                break;
            }
        }
        if no_enemies {
            self.game_ended_state = GameEndedState { game_ended: true, won: true };
            return;
        }

        // check for defeat
        let mut has_bullets = false;
        for renderable_layer in &self.dynamic_renderables {
            for renderable in renderable_layer {
                if renderable.borrow().get_object_type() == ObjectType::Bullet {
                    has_bullets = true;
                }
            }
        }
        if self.player.borrow().has_shot && !has_bullets {
            self.game_ended_state = GameEndedState { game_ended: true, won: false };
            return;
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

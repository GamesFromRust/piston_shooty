use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use std::rc::Rc;
use std::cell::RefCell;
use ears::*;
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

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;
const GUN_SCALE: f64 = 0.5;
const BULLET_SCALE:f64 = 0.03125;

const ENEMY_LAYER: usize = 1;
const PROJECTILE_LAYER: usize = 2;

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

pub enum WorldRequestType {
    AddUpdatable,
    AddDynamicRenderable,
}

pub struct WorldReq {
    renderable: Option<Rc<RefCell<Renderable>>>,
    updatable: Option<Rc<RefCell<Updatable>>>,
    req_type: WorldRequestType,
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

pub struct Ground {
    pub renderable_object: RenderableObject,
}

impl Renderable for Ground {
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
        ObjectType::Ground
    }
}

pub struct Wall {
    pub renderable_object: RenderableObject,
}

impl Renderable for Wall {
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
        ObjectType::Wall
    }
}

pub struct Gun {
    renderable_object: RenderableObject,
    velocity: Vector2,
    should_delete: bool,
}

impl Renderable for Gun {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Gun
    }
}

impl Updatable for Gun {
    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.renderable_object.position += self.velocity * args.dt;
        self.renderable_object.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        Vec::new()
    }

    fn get_should_delete_updatable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_updatable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }
}

impl Gun {
    fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Bullet {
        let velocity = Vector2 {
            x: self.renderable_object.rotation.cos(),
            y: self.renderable_object.rotation.sin(),
        };

        Bullet {
            renderable_object: RenderableObject {
                position: self.renderable_object.position,
                texture: bullet_texture.clone(),
                rotation: self.renderable_object.rotation,
                scale: BULLET_SCALE,
            },
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
            should_delete: false,
        }
    }
}

pub struct Bullet {
    renderable_object: RenderableObject,
    velocity: Vector2,
    should_delete: bool,
}

impl Renderable for Bullet {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Bullet
    }
}

impl Updatable for Bullet {

    #[allow(unused_variables)]
    fn update(&mut self,
                key_states: &HashMap<Key, input::ButtonState>,
                mouse_states: &HashMap<MouseButton, input::ButtonState>,
                mouse_pos: &Vector2,
                args: &UpdateArgs) -> Vec<WorldReq> {
        self.renderable_object.position += self.velocity * args.dt;
        Vec::new()
    }

    fn get_should_delete_updatable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_updatable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }
}

pub struct GameEndedState {
    pub game_ended: bool,
    pub won: bool,
}

pub struct Enemy {
    pub renderable_object: RenderableObject,
    pub should_delete: bool,
}

impl Renderable for Enemy {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
    
    fn get_should_delete_renderable(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete_renderable(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::Enemy
    }
}

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
            Some(projectile) => projectile.borrow().renderable_object.rotation,
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
            Some(projectile) => projectile.borrow().renderable_object.position + ( velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0,
            None => self.renderable_object.position,
        };

        let projectile = Gun {
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

fn create_aabb_cuboid2(renderable_object: &RenderableObject) -> ncollide_geometry::bounding_volume::AABB<nalgebra::PointBase<f64, nalgebra::U2, nalgebra::MatrixArray<f64, nalgebra::U2, nalgebra::U1>>> {
    let half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(
        renderable_object.texture.get_size().0 as f64 * 0.5 * renderable_object.scale,
        renderable_object.texture.get_size().1 as f64 * 0.5 * renderable_object.scale);
    let cuboid2 = Cuboid2::new(half_extents);
    let cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(renderable_object.position.x, renderable_object.position.y), renderable_object.rotation);
    let aabb_cuboid2 = bounding_volume::aabb(&cuboid2, &cuboid2_pos);
    aabb_cuboid2
}

mod input;
mod vector2;
mod asset_loader;
mod texture_manager;
mod font_manager;
mod sound_manager;

extern crate piston;
extern crate glutin_window;
extern crate time;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate graphics;
extern crate find_folder;
extern crate ears;
extern crate ncollide;
extern crate ncollide_geometry;
extern crate ncollide_math;
extern crate nalgebra;
extern crate csv;

use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use asset_loader::AssetLoader;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use ears::*;
use texture_manager::TextureManager;
use sound_manager::SoundManager;
use font_manager::FontManager;
use std::ops::DerefMut;
use ncollide_geometry::shape::Cuboid2;
use ncollide_geometry::bounding_volume;
use ncollide_geometry::bounding_volume::BoundingVolume;
use std::io::{self, Write};
use csv::index::{Indexed, create_index};
use std::cmp;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;
const GUN_SCALE: f64 = 0.5;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MOVE_SPEED_MAX: f64 = 500.0;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const BULLET_SCALE:f64 = 0.03125;
const GRID_WIDTH: u32 = 32;
const GRID_HEIGHT: u32 = 18;
const CELL_WIDTH: u32 = WIDTH / GRID_WIDTH;
const CELL_HEIGHT: u32 = HEIGHT / GRID_HEIGHT;
const PLAYER_SCALE: f64 = 0.5;
const WALL_SCALE: f64 = 1.0;
const ENEMY_SCALE: f64 = 1.0;
const GROUND_SCALE: f64 = 1.0;

const GROUND_LAYER: usize = 0;
const WALL_LAYER: usize = 0;
const ENEMY_LAYER: usize = 1;
const PLAYER_LAYER: usize = 1;
const PROJECTILE_LAYER: usize = 2;

const LEVEL_LIST: [&'static str; 2] = ["Level2", "Level3"];

// TODO: Add self/guns/bullets to here.
pub struct World {
    static_renderables: Vec<Vec<Rc<Renderable>>>,
    dynamic_renderables: Vec<Vec<Rc<RefCell<Renderable>>>>,
    updatables: Vec<Rc<RefCell<Updatable>>>,
    game_ended_state: GameEndedState,
    player: Rc<RefCell<Player>>,
}

pub enum WorldRequestType {
    AddUpdatable,
    AddStaticRenderable,
    AddDynamicRenderable,
}

pub struct WorldReq {
    renderable: Option<Rc<RefCell<Renderable>>>,
    updatable: Option<Rc<RefCell<Updatable>>>,
    req_type: WorldRequestType,
}

pub trait Updatable {
    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> Vec<WorldReq>;
    // TODO: Deletable?
    fn get_should_delete_updatable(&self) -> bool;
    fn set_should_delete_updatable(&mut self, should_delete: bool);
}

impl World {
    fn add_static_renderable_at_layer(&mut self, renderable: Rc<Renderable>, layer: usize) {
        while self.static_renderables.len() <= layer {
            self.static_renderables.push(Vec::new());
        }
        self.static_renderables[layer].push(renderable);
    }

    fn add_dynamic_renderable_at_layer(&mut self, renderable: Rc<RefCell<Renderable>>, layer: usize) {
        while self.dynamic_renderables.len() <= layer {
            self.dynamic_renderables.push(Vec::new());
        }
        self.dynamic_renderables[layer].push(renderable);
    }

    fn add_updatable(&mut self, updatable: Rc<RefCell<Updatable>>) {
        self.updatables.push(updatable);
    }

    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) {

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
                WorldRequestType::AddStaticRenderable => {},
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

// TODO: Object model.
pub struct RenderableObject {
    position: Vector2,
    rotation: f64,
    scale: f64,
    texture: Rc<G2dTexture>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Wall,
    Bullet,
    Gun,
    Enemy,
    Player,
    Ground,
}

pub trait Renderable {
    fn get_renderable_object(&self) -> &RenderableObject;
    fn get_should_delete_renderable(&self) -> bool;
    fn set_should_delete_renderable(&mut self, should_delete: bool);
    fn get_object_type(&self) -> ObjectType;
}

pub struct Ground {
    renderable_object: RenderableObject,
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
    renderable_object: RenderableObject,
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
    game_ended: bool,
    won: bool,
}

pub struct Enemy {
    renderable_object: RenderableObject,
    should_delete: bool,
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
    renderable_object: RenderableObject,
    guns: Vec<Rc<RefCell<Gun>>>,
    gun_texture: Rc<G2dTexture>,
    gun_sound: Rc<RefCell<Sound>>,
    bullet_texture: Rc<G2dTexture>,
    bullet_sound: Rc<RefCell<Sound>>,
    has_shot: bool,
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
        let mut world_reqs: Vec<WorldReq> = Vec::new();

        if self.has_shot {
            return world_reqs;
        }

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

        self.has_shot = true;

        world_reqs
    }

    fn shoot_gun(&mut self, mouse_pos: &Vector2) -> Vec<WorldReq>  {
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

    fn apply_input(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, dt: f64) -> Vec<WorldReq> {
        let mut player_velocity: Vector2 = Vector2::default();
        let mut world_reqs: Vec<WorldReq> = Vec::new();

        for (key, value) in key_states {
            match *key {
                // self
                Key::W => {
                    if value.pressed || value.held {
                        player_velocity.y -= 1.0 * dt;
                    }
                }
                Key::A => {
                    if value.pressed || value.held {
                        player_velocity.x -= 1.0 * dt;
                    }
                }
                Key::S => {
                    if value.pressed || value.held {
                        player_velocity.y += 1.0 * dt;
                    }
                }
                Key::D => {
                    if value.pressed || value.held {
                        player_velocity.x += 1.0 * dt;
                    }
                }
                // Default
                _ => {}
            }
        }

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

        if player_velocity != Vector2::default() {
            player_velocity.normalize();
            self.renderable_object.position += player_velocity * MOVE_SPEED_MAX * dt;
        }

        world_reqs
    }
}

pub struct App {
    window: piston_window::PistonWindow,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    font_manager: FontManager,
    window_height: f64,
    window_width: f64,
    is_paused: bool,
    world: World,
    texture_manager: TextureManager,
    sound_manager: SoundManager,
    level_index: usize,
}

fn draw_victory_overlay(font_manager: &mut FontManager, c: &Context, gl: &mut G2d, window_width: f64, window_height: f64) {
    let victory_text = "Success!";
    let transform = c.transform.trans(window_width * 0.5, window_height * 0.5);
    let cache_rc = font_manager.get("Roboto-Regular.ttf");
    text(WHITE, 36, &victory_text, cache_rc.borrow_mut().deref_mut(), transform, gl);
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

fn create_aabb_cuboid2(renderable_object: &RenderableObject) -> ncollide_geometry::bounding_volume::AABB<nalgebra::PointBase<f64, nalgebra::U2, nalgebra::MatrixArray<f64, nalgebra::U2, nalgebra::U1>>> {
    let half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(
        renderable_object.texture.get_size().0 as f64 * 0.5 * renderable_object.scale,
        renderable_object.texture.get_size().1 as f64 * 0.5 * renderable_object.scale);
    let cuboid2 = Cuboid2::new(half_extents);
    let cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(renderable_object.position.x, renderable_object.position.y), renderable_object.rotation);
    let aabb_cuboid2 = bounding_volume::aabb(&cuboid2, &cuboid2_pos);
    aabb_cuboid2
}

impl App {
    fn render(&mut self, event: &Input) {
        // TODO: Read a book on how to do a fps counter.
        let curr_frame_time: u64 = time::precise_time_ns();

        self.num_frames_in_batch += 1;

        if curr_frame_time >= self.last_batch_start_time + NSEC_PER_SEC {
            self.average_frame_time = (curr_frame_time - self.last_batch_start_time) /
                                      self.num_frames_in_batch;
            self.last_batch_start_time = curr_frame_time;
            self.num_frames_in_batch = 0;
        }

        let fps = NSEC_PER_SEC / self.average_frame_time;
        let fps_text =
            "FPS: ".to_string() + &fps.to_string() + &"\naverage_frame_time: ".to_string() +
            &self.average_frame_time.to_string() +
            &"\nnum_frames_in_batch: ".to_string() +
            &self.num_frames_in_batch.to_string() +
            &"\nlast_batch_start_time: ".to_string() +
            &self.last_batch_start_time.to_string() +
            &"\ncurr_frame_time: ".to_string() + &curr_frame_time.to_string();

        let mut font_manager = &mut self.font_manager;
        let window_width = self.window_width;
        let window_height = self.window_height;
        let game_ended_state = &self.world.game_ended_state;
        let world = &self.world;

        self.window.draw_2d(event, |c: Context, mut gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            let max_layers = cmp::max(world.static_renderables.len(), world.dynamic_renderables.len());
            for i in 0..max_layers {
                if i < world.static_renderables.len() {
                    for renderable in &world.static_renderables[i] {
                        let renderable_object = renderable.get_renderable_object();
                        render_renderable_object(&c, &mut gl, &renderable_object);
                    }
                }
                if i < world.dynamic_renderables.len() {
                    for renderable in &world.dynamic_renderables[i] {
                        // TODO: Why can't we do this?
                        // let renderable_object = renderable.borrow().get_renderable_object();
                        // render_renderable_object(&c, &mut gl, &renderable_object);
                        render_renderable_object(&c, &mut gl, &renderable.borrow().get_renderable_object());
                    }
                }
            }

            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
            let cache_rc = font_manager.get("Roboto-Regular.ttf");
            text(WHITE, 14, &fps_text, cache_rc.borrow_mut().deref_mut(), transform, gl);

            if game_ended_state.game_ended && game_ended_state.won {
                draw_victory_overlay(&mut font_manager, &c, &mut gl, window_width, window_height);
            }
        });
    }

    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) {
        if self.is_paused {
            return;
        }

        self.world.update(&key_states, &mouse_states, &mouse_pos, &args);

        if self.world.game_ended_state.game_ended == true {
            if self.world.game_ended_state.won == false {
                self.world = load_level(&mut self.texture_manager, &mut self.sound_manager, LEVEL_LIST[self.level_index]);
                return;
            }
            
            self.level_index = self.level_index + 1;
            if self.level_index >= LEVEL_LIST.len() {
                self.is_paused = true;
            } else {
                self.world = load_level(&mut self.texture_manager, &mut self.sound_manager, LEVEL_LIST[self.level_index]);
            }
        }
    }
}

fn load_level(texture_manager:&mut TextureManager, sound_manager:&mut SoundManager, level_name:&str) -> World {
    let hand_gun = texture_manager.get("textures\\hand-gun_square.png");
    let gun_gun = texture_manager.get("textures\\GunGunV1.png");
    let bullet = texture_manager.get("textures\\bullet.png");
    let wall = texture_manager.get("textures\\brick_square.png");
    let enemy = texture_manager.get("textures\\enemy.png");
    let ground = texture_manager.get("textures\\ground.png");

    let player: Player = Player {
        renderable_object: RenderableObject {
            texture: hand_gun.clone(),
            position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            rotation: 0.0,
            scale: PLAYER_SCALE,
        },
        guns: Vec::new(),
        gun_texture: gun_gun.clone(),
        gun_sound: sound_manager.get("sounds\\boom.ogg"),
        bullet_texture: bullet.clone(),
        bullet_sound: sound_manager.get("sounds\\boop.ogg"),
        has_shot: false,
    };
    
    let player = Rc::new(RefCell::new(player));

    let mut world: World = World {
        static_renderables: Vec::new(),
        dynamic_renderables: Vec::new(),
        updatables: Vec::new(),
        game_ended_state: GameEndedState {
            game_ended: false,
            won: false
        },
        player: player.clone(),
    };

    let new_csv_rdr = || csv::Reader::from_file(format!("assets\\Levels\\{}.csv", level_name)).unwrap().has_headers(false);
    let mut index_data = io::Cursor::new(Vec::new());
    create_index(new_csv_rdr(), index_data.by_ref()).unwrap();
    let mut index = Indexed::open(new_csv_rdr(), index_data).unwrap();

    let mut level: Vec<Vec<String>> = Vec::new();
    for row in index.records() {
        let row = row.unwrap();

        for item in &row {
            print!("{},", item);
        }
        println!("");

        level.push(row);
    }

    assert!(level.len() as u32 == GRID_HEIGHT);
    for row in &level {
        assert!(row.len() as u32 == GRID_WIDTH);
    }

    // Read in a level.
    let mut line_num = 0;
    for line in &level {
        let mut item_num = 0;
        for item in line {
            if item == "W" {
                let wall = Wall {
                    renderable_object: RenderableObject {
                        position: Vector2 {
                            x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                            y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                        },
                        rotation: 0.0,
                        texture: wall.clone(),
                        scale: WALL_SCALE,
                    },
                };
                let rc = Rc::new(wall);
                world.add_static_renderable_at_layer(rc.clone(), WALL_LAYER);
            } else if item == "P" {
                let ground = Ground {
                    renderable_object: RenderableObject {
                        position: Vector2 {
                            x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                            y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                        },
                        rotation: 0.0,
                        texture: ground.clone(),
                        scale: GROUND_SCALE,
                    },
                };
                let rc = Rc::new(ground);
                world.add_static_renderable_at_layer(rc.clone(), GROUND_LAYER);

                player.borrow_mut().renderable_object.position.x = (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64;
                player.borrow_mut().renderable_object.position.y = (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64;

                world.add_dynamic_renderable_at_layer(player.clone(), PLAYER_LAYER);
                world.add_updatable(player.clone());
            } else if item == "E" {
                let ground = Ground {
                    renderable_object: RenderableObject {
                        position: Vector2 {
                            x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                            y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                        },
                        rotation: 0.0,
                        texture: ground.clone(),
                        scale: GROUND_SCALE,
                    },
                };
                let rc = Rc::new(ground);
                world.add_static_renderable_at_layer(rc.clone(), GROUND_LAYER);

                let enemy = Enemy {
                    renderable_object: RenderableObject {
                        position: Vector2 {
                            x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64,
                            y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                        },
                        rotation: 0.0,
                        texture: enemy.clone(),
                        scale: ENEMY_SCALE,
                    },
                    should_delete: false,
                };
                let refcell = Rc::new(RefCell::new(enemy));
                world.add_dynamic_renderable_at_layer(refcell.clone(), ENEMY_LAYER);
            } else if item == "_" {
                let ground = Ground {
                    renderable_object: RenderableObject {
                        position: Vector2 {
                            x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                            y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                        },
                        rotation: 0.0,
                        texture: ground.clone(),
                        scale: GROUND_SCALE,
                    },
                };
                let rc = Rc::new(ground);
                world.add_static_renderable_at_layer(rc.clone(), GROUND_LAYER);
            }
            item_num += 1;
        }
        line_num += 1;
    }
    world
}

fn main() {
    let window_settings = WindowSettings::new("piston_shooty", [WIDTH, HEIGHT]);

    let assets_path: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let window: piston_window::PistonWindow = window_settings.exit_on_esc(true)
        .build()
        .unwrap();

    let asset_loader = AssetLoader {
        assets_path: assets_path,
        factory: window.factory.clone(),
    };
    let asset_loader = Rc::new(asset_loader);

    let mut font_manager = FontManager {
        asset_loader: asset_loader.clone(),
        fonts_by_filename: HashMap::new(),
    };
    
    let mut texture_manager = TextureManager {
        asset_loader: asset_loader.clone(),
        textures_by_filename: HashMap::new(),
    };

    let mut sound_manager = SoundManager {
        asset_loader: asset_loader.clone(),
        sounds_by_filename: HashMap::new(),
    };

    font_manager.get("Roboto-Regular.ttf");

    let world = load_level(&mut texture_manager, &mut sound_manager, "Level2"); 

    let mut app = App {
        window: window,
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        font_manager: font_manager,
        window_height: HEIGHT as f64,
        window_width: WIDTH as f64,
        is_paused: false,
        world: world,
        texture_manager: texture_manager,
        sound_manager: sound_manager,
        level_index: 0,
    };
    app.window.set_max_fps(u64::max_value());

    let mut key_states: HashMap<Key, input::ButtonState> = HashMap::new();
    let mut mouse_states: HashMap<MouseButton, input::ButtonState> = HashMap::new();
    let mut mouse_pos = Vector2::default();

    // TODO: Why is args.dt locked to 120fps for UpdateArgs?
    while let Some(e) = app.window.next() {
        // Input.
        input::gather_input(&e, &mut key_states, &mut mouse_states, &mut mouse_pos);
        
        if let Some(u) = e.update_args() {
            if !app.is_paused {
                app.update(&key_states, &mouse_states, &mouse_pos, &u);
                input::update_input(&mut key_states, &mut mouse_states);
            }
        }

        // Render.
        if e.render_args().is_some() {
            app.render(&e);
        }
    }
}

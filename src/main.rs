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

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 75.0;
const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
const GUN_ROTATIONAL_VELOCITY: f64 = 4.0;
const GUN_SCALE: f64 = 0.5;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
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

// TODO: Object model.
pub struct RenderableObject {
    position: Vector2,
    rotation: f64,
    scale: f64,
    texture: Rc<G2dTexture>,
}

pub struct Ground {
    renderable_object: RenderableObject,
}

pub struct Wall {
    renderable_object: RenderableObject,
}

pub struct Projectile {
    renderable_object: RenderableObject,
    velocity: Vector2,
}

pub struct GameEndedState {
    game_ended: bool,
    won: bool,
}

impl Projectile {
    fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Projectile {
        let velocity = Vector2 {
            x: self.renderable_object.rotation.cos(),
            y: self.renderable_object.rotation.sin(),
        };

        Projectile {
            renderable_object: RenderableObject {
                position: self.renderable_object.position,
                texture: bullet_texture.clone(),
                rotation: self.renderable_object.rotation,
                scale: BULLET_SCALE,
            },
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
        }
    }
}

pub struct Enemy {
    renderable_object: RenderableObject,
}

pub struct Player {
    renderable_object: RenderableObject,
    projectiles: Vec<Projectile>, // guns
    projectile_texture: Rc<G2dTexture>,
    projectile_sound: Rc<RefCell<Sound>>,
    bullet_texture: Rc<G2dTexture>,
    bullets: Vec<Projectile>,
    bullet_sound: Rc<RefCell<Sound>>,
}

impl Player {
    fn shoot_bullets(&mut self) {
        for projectile in &self.projectiles {
            self.bullets.push(projectile.shoot_bullet(&self.bullet_texture));
            self.bullet_sound.borrow_mut().play();
        }    
    }

    fn shoot_gun(&mut self, mouse_pos: &Vector2) {
        let rotation = match self.projectiles.last() {
            Some(projectile) => projectile.renderable_object.rotation,
            None => self.renderable_object.rotation,
        };

        let velocity = match self.projectiles.last() {
            Some(_) => {
                let vel = Vector2 {
                    x: rotation.cos(),
                    y: rotation.sin(),
                };
                vel * PROJECTILE_VELOCITY_MAGNITUDE
            },
            None => (*mouse_pos - self.renderable_object.position).normalized() * PROJECTILE_VELOCITY_MAGNITUDE,
        };

        let position = match self.projectiles.last() {
            Some(projectile) => projectile.renderable_object.position + ( velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0,
            None => self.renderable_object.position,
        };

        let projectile = Projectile {
            renderable_object: RenderableObject {
                position: position,
                texture: self.projectile_texture.clone(),
                rotation: rotation,
                scale: GUN_SCALE,
            },
            velocity: velocity,
        };

        self.projectile_sound.borrow_mut().play();

        self.projectiles.push(projectile);
    }

    fn update(&mut self, mouse_pos: &Vector2, args: &UpdateArgs) {
        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.renderable_object.position;
        self.renderable_object.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        // Move our projectiles.
        for projectile in &mut self.projectiles {
            projectile.renderable_object.position += projectile.velocity * args.dt;
            projectile.renderable_object.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        }

        // Move our bullets.
        for bullet in &mut self.bullets {
            bullet.renderable_object.position += bullet.velocity * args.dt;
        }
    }
}

pub struct App {
    window: piston_window::PistonWindow,
    player: Player,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    font_manager: FontManager,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    grounds: Vec<Ground>,
    game_ended_state: GameEndedState,
    window_height: f64,
    window_width: f64,
    is_paused: bool
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

        let player = &self.player;
        let mut font_manager = &mut self.font_manager;
        let enemies = &self.enemies;
        let walls = &self.walls;
        let grounds = &self.grounds;
        let window_width = self.window_width;
        let window_height = self.window_height;
        let game_ended_state = &self.game_ended_state;

        self.window.draw_2d(event, |c: Context, mut gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw our walls.
            for wall in walls {
                render_renderable_object(&c, &mut gl, &wall.renderable_object);
            }

            // Draw our grounds.
            for ground in grounds {
                render_renderable_object(&c, &mut gl, &ground.renderable_object);
            }
            
            // Draw our projectiles.
            for projectile in &player.projectiles {
                render_renderable_object(&c, &mut gl, &projectile.renderable_object);
            }

            // Draw our bullets.
            for bullet in &player.bullets {
                render_renderable_object(&c, &mut gl, &bullet.renderable_object);
            }

            // Debug rectangle.
            match player.projectiles.last() {
                Some(projectile) => {
                    let transform = c.transform
                        .trans(projectile.renderable_object.position.x, projectile.renderable_object.position.y)
                        .rot_rad(projectile.renderable_object.rotation)
                        .trans((projectile.renderable_object.texture.get_size().0 as f64) * -0.5 * GUN_SCALE,
                                (projectile.renderable_object.texture.get_size().1 as f64) * -0.5 * GUN_SCALE)
                        .scale(GUN_SCALE, GUN_SCALE);
                    
                    let rect: graphics::types::Rectangle = [0.0, 0.0, 10000.0, 1.0];
                    rectangle(RED, rect, transform, gl);
                },
                None => (),
            }

            for enemy in enemies {
                render_renderable_object(&c, &mut gl, &enemy.renderable_object);
            }

            render_renderable_object(&c, &mut gl, &player.renderable_object);

            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
            let cache_rc = font_manager.get("Roboto-Regular.ttf");
            text(WHITE, 14, &fps_text, cache_rc.borrow_mut().deref_mut(), transform, gl);

            if game_ended_state.game_ended && game_ended_state.won {
                draw_victory_overlay(&mut font_manager, &c, &mut gl, window_width, window_height);
            }
        });
    }

    fn update(&mut self, mouse_pos: &Vector2, args: &UpdateArgs) {
        if self.enemies.is_empty() {
            self.game_ended_state = GameEndedState { game_ended: true, won: true };
            self.is_paused = true;
            return;
        }

        self.player.update(mouse_pos, args);

        let bullets = &mut self.player.bullets;
        let enemies = &mut self.enemies;
        let walls = &self.walls;
        let projectiles = &mut self.player.projectiles;

        bullets.retain(|ref bullet| {
            let bullet_aabb_cuboid2 = create_aabb_cuboid2(&bullet.renderable_object);

            let mut intersected = false;

            enemies.retain(|ref enemy| {
                let enemy_aabb_cuboid2 = create_aabb_cuboid2(&enemy.renderable_object);

                let intersects = enemy_aabb_cuboid2.intersects(&bullet_aabb_cuboid2);
                intersected = intersects || intersected;
                !intersects
            });

            for wall in walls {
                let wall_aabb_cuboid2 = create_aabb_cuboid2(&wall.renderable_object);

                let intersects = wall_aabb_cuboid2.intersects(&bullet_aabb_cuboid2);
                intersected = intersects || intersected;
            }

            !intersected
        });

        projectiles.retain(|ref gun| {
            let gun_aabb_cuboid2 = create_aabb_cuboid2(&gun.renderable_object);

            let mut intersected = false;

            for wall in walls {
                let wall_aabb_cuboid2 = create_aabb_cuboid2(&wall.renderable_object);

                let intersects = wall_aabb_cuboid2.intersects(&gun_aabb_cuboid2);
                intersected = intersects || intersected;
            }

            !intersected
        });
    }
}

fn apply_input(player: &mut Player,
               key_states: &HashMap<Key, input::ButtonState>,
               mouse_states: &HashMap<MouseButton, input::ButtonState>,
               mouse_pos: &Vector2,
               dt: f64) {
    let mut player_velocity: Vector2 = Vector2::default();

    for (key, value) in key_states {
        match *key {
            // Player
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
                    player.shoot_gun(mouse_pos);
                }
            },
            MouseButton::Right => {
                if value.pressed {
                    player.shoot_bullets();
                }
            }
            // Default
            _ => {}
        }
    }

    if player_velocity == Vector2::default() {
        return;
    }
    player_velocity.normalize();
    player.renderable_object.position += player_velocity * MOVE_SPEED_MAX * dt;
}

fn main() {
    let window_settings = WindowSettings::new("piston_shooty", [WIDTH, HEIGHT]);

    let assets_path: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let window: piston_window::PistonWindow = window_settings.exit_on_esc(true)
        .build()
        .unwrap();

    let new_csv_rdr = || csv::Reader::from_file("assets\\Levels\\Level2.csv").unwrap().has_headers(false);
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

    let hand_gun = texture_manager.get("textures\\hand-gun_square.png");
    let gun_gun = texture_manager.get("textures\\GunGunV1.png");
    let bullet = texture_manager.get("textures\\bullet.png");
    let wall = texture_manager.get("textures\\brick_square.png");
    let enemy = texture_manager.get("textures\\enemy.png");
    let ground = texture_manager.get("textures\\ground.png");

    font_manager.get("Roboto-Regular.ttf");

    let mut enemies:Vec<Enemy> = Vec::new();
    let mut walls: Vec<Wall> = Vec::new();
    let mut grounds: Vec<Ground> = Vec::new();
    let mut player: Player = Player {
        renderable_object: RenderableObject {
            texture: hand_gun.clone(),
            position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            rotation: 0.0,
            scale: PLAYER_SCALE,
        },
        projectiles: Vec::new(),
        projectile_texture: gun_gun.clone(),
        projectile_sound: sound_manager.get("sounds\\boom.ogg"),
        bullet_texture: bullet.clone(),
        bullets: Vec::new(),
        bullet_sound: sound_manager.get("sounds\\boop.ogg"),
    };

    // Read in a level.
    let mut line_num = 0;
    for line in &level {
        let mut item_num = 0;
        for item in line {
            if item == "W" {
                walls.push(
                    Wall {
                        renderable_object: RenderableObject {
                            position: Vector2 {
                                x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                                y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                            },
                            rotation: 0.0,
                            texture: wall.clone(),
                            scale: WALL_SCALE,
                        },
                    });
            } else if item == "P" {
                player.renderable_object.position = Vector2 {
                    x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64,
                    y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                };
            } else if item == "E" {
                enemies.push(
                    Enemy {
                        renderable_object: RenderableObject {
                            position: Vector2 {
                                x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64,
                                y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                            },
                            rotation: 0.0,
                            texture: enemy.clone(),
                            scale: ENEMY_SCALE,
                        },
                    });
            } else if item == "_" {
                grounds.push(
                    Ground {
                        renderable_object: RenderableObject {
                            position: Vector2 {
                                x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                                y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                            },
                            rotation: 0.0,
                            texture: ground.clone(),
                            scale: GROUND_SCALE,
                        },
                    });
            }
            item_num += 1;
        }
        line_num += 1;
    }

    let mut app = App {
        window: window,
        player: player,
        enemies: enemies,
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        font_manager: font_manager,
        walls: walls,
        grounds: grounds,
        game_ended_state: GameEndedState {
            game_ended: false,
            won: false
        },
        window_height: HEIGHT as f64,
        window_width: WIDTH as f64,
        is_paused: false
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
                apply_input(&mut app.player,
                            &key_states,
                            &mouse_states,
                            &mouse_pos,
                            u.dt);

                input::update_input(&mut key_states, &mut mouse_states);
                app.update(&mouse_pos, &u);
            }
        }

        // Render.
        if e.render_args().is_some() {
            app.render(&e);
        }
    }
}

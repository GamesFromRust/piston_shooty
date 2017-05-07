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

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 100.0;
const BULLET_VELOCITY_MAGNITUDE: f64 = 200.0;
const GUN_ROTATIONAL_VELOCITY: f64 = 2.5;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MOVE_SPEED_MAX: f64 = 500.0;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const BULLET_SCALE:f64 = 0.03125;

pub struct Wall {
    position: Vector2,
    rotation: f64,
    texture: Rc<G2dTexture>,
}

pub struct Projectile {
    position: Vector2,
    velocity: Vector2,
    rotation: f64,
    texture: Rc<G2dTexture>,
}

impl Projectile {
    fn shoot_bullet(&self, bullet_texture: &Rc<G2dTexture>) -> Projectile {
        let velocity = Vector2 {
            x: self.rotation.cos(),
            y: self.rotation.sin(),
        };

        Projectile {
            position: self.position,
            velocity: velocity * BULLET_VELOCITY_MAGNITUDE,
            rotation: self.rotation,
            texture: bullet_texture.clone(),
        }
    }
}

pub struct Enemy {
    position: Vector2,
    rotation: f64,
    texture: Rc<G2dTexture>,
}

pub struct Player {
    position: Vector2,
    rotation: f64,
    projectiles: Vec<Projectile>, // guns
    tex: Rc<G2dTexture>,
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
            Some(u) => u.rotation,
            None => self.rotation,
        };

        let velocity = match self.projectiles.last() {
            Some(_) => {
                let vel = Vector2 {
                    x: rotation.cos(),
                    y: rotation.sin(),
                };
                vel * PROJECTILE_VELOCITY_MAGNITUDE
            },
            None => (*mouse_pos - self.position).normalized() * PROJECTILE_VELOCITY_MAGNITUDE,
        };

        let position = match self.projectiles.last() {
            Some(u) => u.position + ( velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0,
            None => self.position,
        };

        let projectile = Projectile {
            position: position,
            velocity: velocity,
            rotation: rotation,
            texture: self.projectile_texture.clone(),
        };

        self.projectile_sound.borrow_mut().play();

        self.projectiles.push(projectile);
    }

    fn update(&mut self, mouse_pos: &Vector2, args: &UpdateArgs) {
        // Rotate to face our mouse.
        let player_to_mouse = *mouse_pos - self.position;
        self.rotation = player_to_mouse.y.atan2(player_to_mouse.x);

        // Move our projectiles.
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * args.dt;
            projectile.rotation += GUN_ROTATIONAL_VELOCITY * args.dt;
        }

        // Move our bullets.
        for bullet in &mut self.bullets {
            bullet.position += bullet.velocity * args.dt;
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
        let font_manager = &mut self.font_manager;
        let enemies = &self.enemies;
        let walls = &self.walls;

        self.window.draw_2d(event, |c: Context, gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            let player_texture = player.tex.deref();

            let scale = 0.5;
            let transform = c.transform
                .trans(player.position.x, player.position.y)
                .rot_rad(player.rotation)
                .trans((player_texture.get_size().0 as f64) * -0.5 * scale,
                       (player_texture.get_size().1 as f64) * -0.5 * scale)
                .scale(scale, scale);

            // Set our player sprite position.
            image(player_texture.deref(), transform, gl);
            
            // Draw our projectiles.
            for projectile in &player.projectiles {
                let transform = c.transform
                    .trans(projectile.position.x, projectile.position.y)
                    .rot_rad(projectile.rotation)
                    .trans((projectile.texture.get_size().0 as f64) * -0.5,
                           (projectile.texture.get_size().1 as f64) * -0.5);
                image(projectile.texture.deref(), transform, gl);
            }

            // Draw our bullets.
            for bullet in &player.bullets {
                let transform = c.transform
                    .trans(bullet.position.x, bullet.position.y)
                    .rot_rad(bullet.rotation)
                    .trans((bullet.texture.get_size().0 as f64) * -0.5 * BULLET_SCALE,
                           (bullet.texture.get_size().1 as f64) * -0.5 * BULLET_SCALE)
                    .scale(BULLET_SCALE, BULLET_SCALE);
                // let rect: graphics::types::Rectangle = [0.0, 0.0, bullet.texture.get_size().0 as f64, bullet.texture.get_size().1 as f64];
                // rectangle(RED, rect, transform, gl);
                image(bullet.texture.deref(), transform, gl);
            }

            // Draw our wall.
            for wall in walls {
                let transform = c.transform
                    .trans(wall.position.x, wall.position.y)
                    .rot_rad(wall.rotation)
                    .trans((wall.texture.get_size().0 as f64) * -0.5,
                           (wall.texture.get_size().1 as f64) * -0.5);
                image(wall.texture.deref(), transform, gl);
            }

            // Debug rectangle.
            match player.projectiles.last() {
                Some(projectile) => {
                    let transform = c.transform
                        .trans(projectile.position.x, projectile.position.y)
                        .rot_rad(projectile.rotation);
                    let rect: graphics::types::Rectangle = [0.0, 0.0, 10000.0, 1.0];
                    rectangle(RED, rect, transform, gl);
                },
                None => (),
            }

            let enemy_scale = 1.0;
            for enemy in enemies {
                // draw a debug rect
                let transform = c.transform
                    .trans(enemy.position.x, enemy.position.y)
                    .rot_rad(enemy.rotation)
                    .trans((enemy.texture.get_size().0 as f64) * -0.5 * enemy_scale,
                           (enemy.texture.get_size().1 as f64) * -0.5 * enemy_scale)
                    .scale(enemy_scale, enemy_scale);
                // let rect: graphics::types::Rectangle = [0.0, 0.0, enemy.texture.get_size().0 as f64, enemy.texture.get_size().1 as f64];
                // rectangle(RED, rect, transform, gl);
                image(enemy.texture.deref(), transform, gl);
            }


            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
            let cache_rc = font_manager.get("Roboto-Regular.ttf");
            text(WHITE, 14, &fps_text, cache_rc.borrow_mut().deref_mut(), transform, gl);
        });
    }

    fn update(&mut self, mouse_pos: &Vector2, args: &UpdateArgs) {
        self.player.update(mouse_pos, args);

        let bullets = &mut self.player.bullets;
        let enemies = &mut self.enemies;
        let walls = &self.walls;

        bullets.retain(|ref bullet| {
            let bullet_half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(bullet.texture.get_size().0 as f64 * 0.5 * BULLET_SCALE, bullet.texture.get_size().1 as f64 * 0.5 * BULLET_SCALE);
            let bullet_cuboid2 = Cuboid2::new(bullet_half_extents);
            let bullet_cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(bullet.position.x, bullet.position.y), bullet.rotation);
            let bullet_aabb_cuboid2 = bounding_volume::aabb(&bullet_cuboid2, &bullet_cuboid2_pos);

            let mut intersected = false;

            enemies.retain(|ref enemy| {
                let enemy_half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(enemy.texture.get_size().0 as f64 * 0.5, enemy.texture.get_size().1 as f64 * 0.5);
                let enemy_cuboid2 = Cuboid2::new(enemy_half_extents);
                let enemy_cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(enemy.position.x, enemy.position.y), enemy.rotation);
                let enemy_aabb_cuboid2 = bounding_volume::aabb(&enemy_cuboid2, &enemy_cuboid2_pos);

                let intersects = enemy_aabb_cuboid2.intersects(&bullet_aabb_cuboid2);
                intersected = intersects || intersected;
                !intersects
            });

            for wall in walls {
                let wall_half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(wall.texture.get_size().0 as f64 * 0.5, wall.texture.get_size().1 as f64 * 0.5);
                let wall_cuboid2 = Cuboid2::new(wall_half_extents);
                let wall_cuboid2_pos = nalgebra::geometry::Isometry2::new(nalgebra::core::Vector2::new(wall.position.x, wall.position.y), wall.rotation);
                let wall_aabb_cuboid2 = bounding_volume::aabb(&wall_cuboid2, &wall_cuboid2_pos);

                let intersects = wall_aabb_cuboid2.intersects(&bullet_aabb_cuboid2);
                intersected = intersects || intersected;
            }

            !intersected
        })
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
    player.position += player_velocity * MOVE_SPEED_MAX * dt;
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

    let hand_gun = texture_manager.get("textures\\hand-gun.png");
    let gun_gun = texture_manager.get("textures\\GunGunV1.png");
    let bullet = texture_manager.get("textures\\bullet.png");
    let wall = texture_manager.get("textures\\brick.png");

    font_manager.get("Roboto-Regular.ttf");

    let mut enemies:Vec<Enemy> = Vec::new();
    enemies.push(
        Enemy {
            position: Vector2 { x: (WIDTH / 2) as f64, y: (HEIGHT / 2) as f64 },
            rotation: 0.0,
            texture: texture_manager.get("textures\\enemy.png"),
        });

    let mut walls: Vec<Wall> = Vec::new();
    walls.push(
        Wall {
            position: Vector2 { x: 50 as f64, y: 50 as f64 },
            rotation: 0.0,
            texture: wall.clone(),
        });

    let mut app = App {
        window: window,
        player: Player {
            position: Vector2 { x: 1.0, y: 1.0 },
            rotation: 0.0,
            projectiles: Vec::new(),
            tex: hand_gun.clone(),
            projectile_texture: gun_gun.clone(),
            projectile_sound: sound_manager.get("sounds\\boom.ogg"),
            bullet_texture: bullet.clone(),
            bullets: Vec::new(),
            bullet_sound: sound_manager.get("sounds\\boop.ogg"),
        },
        enemies: enemies,
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        font_manager: font_manager,
        walls: walls,
    };
    app.window.set_max_fps(u64::max_value());

    let mut key_states: HashMap<Key, input::ButtonState> = HashMap::new();
    let mut mouse_states: HashMap<MouseButton, input::ButtonState> = HashMap::new();
    let mut mouse_pos = Vector2::default();

    while let Some(e) = app.window.next() {
        // Input.
        input::gather_input(&e, &mut key_states, &mut mouse_states, &mut mouse_pos);
        if let Some(u) = e.update_args() {
            apply_input(&mut app.player,
                        &key_states,
                        &mouse_states,
                        &mouse_pos,
                        u.dt);
            input::update_input(&mut key_states, &mut mouse_states);
        }

        // Update.
        if let Some(u) = e.update_args() {
            app.update(&mouse_pos, &u);
        }

        // Render.
        if e.render_args().is_some() {
            app.render(&e);
        }
    }
}

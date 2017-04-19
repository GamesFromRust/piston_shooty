mod input;
mod vector2;
mod asset_loader;
mod texture_manager;
mod font_manager;

extern crate piston;
extern crate glutin_window;
extern crate time;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate graphics;
extern crate find_folder;
extern crate ears;

use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use asset_loader::AssetLoader;
use std::rc::Rc;
use std::ops::Deref;
use ears::*;
use texture_manager::TextureManager;
use font_manager::FontManager;
use std::ops::DerefMut;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 100.0;
const GUN_ROTATIONAL_VELOCITY: f64 = 2.5;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MOVE_SPEED_MAX: f64 = 500.0;

const NSEC_PER_SEC: u64 = 1_000_000_000;

pub struct Projectile {
    position: Vector2,
    velocity: Vector2,
    rotation: f64,
    texture: Rc<G2dTexture>,
}

impl Projectile {
    fn shoot_bullet(&self) {
        // let bullet = Projectile {
        //     position: position,
        //     velocity: velocity,
        //     rotation: rotation,
        //     texture: self.bullet_texture.clone(),
        // };
        println!("pew");
    }
}

pub struct Player {
    position: Vector2,
    rotation: f64,
    projectiles: Vec<Projectile>,
    tex: Rc<G2dTexture>,
    projectile_texture: Rc<G2dTexture>,
    projectile_sound: Sound,
}

impl Player {
    fn shoot_bullet(&self) {
        for projectile in &self.projectiles {
            projectile.shoot_bullet();
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

        self.projectile_sound.play();

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
    }
}

pub struct App {
    window: piston_window::PistonWindow,
    player: Player,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    font_manager: FontManager,
    texture_manager: TextureManager,
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

        self.window.draw_2d(event, |c: Context, gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
            let cache_rc = font_manager.get("Roboto-Regular.ttf");
            text(WHITE, 14, &fps_text, cache_rc.borrow_mut().deref_mut(), transform, gl);

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
        });
    }

    fn update(&mut self, mouse_pos: &Vector2, args: &UpdateArgs) {
        // Update our player.
        self.player.update(mouse_pos, args);
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
                    println!("Right clickity");
                    player.shoot_bullet();
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
    let width = 800;
    let height = 800;

    let window_settings = WindowSettings::new("piston_shooty", [width, height]);

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

    let font_manager = FontManager {
        asset_loader: asset_loader.clone(),
        fonts_by_filename: HashMap::new(),
    };
    
    let mut texture_manager = TextureManager {
        asset_loader: asset_loader.clone(),
        textures_by_filename: HashMap::new(),
    };

    let hand_gun = texture_manager.get("hand-gun.png");
    let gun_gun = texture_manager.get("GunGunV1.png");

    let mut app = App {
        window: window,
        player: Player {
            position: Vector2 { x: 1.0, y: 1.0 },
            rotation: 0.0,
            projectiles: Vec::new(),
            tex: hand_gun.clone(),
            projectile_texture: gun_gun.clone(),
            projectile_sound: Sound::new("D:\\Development\\Rust\\piston_shooty\\assets\\sounds\\boom.ogg").unwrap()
        },
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        font_manager: font_manager,
        texture_manager: texture_manager
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

mod input;
mod vector2;
mod asset_loader;

extern crate piston;
extern crate glutin_window;
extern crate time;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate graphics;
extern crate find_folder;

use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use asset_loader::AssetLoader;
use std::rc::Rc;
use std::ops::Deref; 

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 300.0;
const PLAYER_ROTATIONAL_VELOCITY: f64 = 5.0;
const GREEN:    [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE:    [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MOVE_SPEED_MAX: f64 = 500.0;

const NSEC_PER_SEC: u64 = 1_000_000_000;

// pub struct Team {
//     team_color: [f32; 4]
// }

//const NO_TEAM: Team = Team {team_color: GREEN};
// const TEAM1: Team = Team {team_color: BLUE};
// const TEAM2: Team = Team {team_color: RED};

pub struct Projectile {
    position: Vector2,
    velocity: Vector2,
    rotation: f64,
}

pub struct Player {
    // team: Team,
    position: Vector2,
    rotation: f64,
    projectiles: Vec<Projectile>,
    tex : G2dTexture,
    projectile_texture: Rc<G2dTexture>,
}

impl Player {
    fn shoot(&mut self) {
        let x = self.rotation.cos();
        let y = self.rotation.sin();
        let mut vel = Vector2 {x: x, y: y};
        vel *= PROJECTILE_VELOCITY_MAGNITUDE;

        let projectile = Projectile {
            position: self.position,
            velocity: vel,
            rotation: 0.0,
        };

        self.projectiles.push(projectile);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += PLAYER_ROTATIONAL_VELOCITY * args.dt;
        // Move our projectiles.
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * args.dt;
            projectile.rotation += PLAYER_ROTATIONAL_VELOCITY * args.dt;
        }
    }
}

pub struct App {
    window: piston_window::PistonWindow,
    players: Vec<Player>,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    assets: std::path::PathBuf
}

impl App {
    fn render(&mut self, event: &Input) {
        // TODO: Read a book on how to do a fps counter.
        let curr_frame_time:u64 = time::precise_time_ns();

        self.num_frames_in_batch += 1;

        if curr_frame_time >= self.last_batch_start_time + NSEC_PER_SEC {
            self.average_frame_time = (curr_frame_time - self.last_batch_start_time) / self.num_frames_in_batch;
            self.last_batch_start_time = curr_frame_time;
            self.num_frames_in_batch = 0;
        }

        let fps = NSEC_PER_SEC / self.average_frame_time;
        let fps_text = "FPS: ".to_string() + &fps.to_string()
        + &"\naverage_frame_time: ".to_string() + &self.average_frame_time.to_string()
        + &"\nnum_frames_in_batch: ".to_string() + &self.num_frames_in_batch.to_string()
        + &"\nlast_batch_start_time: ".to_string() + &self.last_batch_start_time.to_string()
        + &"\ncurr_frame_time: ".to_string() + &curr_frame_time.to_string();

        let square = rectangle::square(0.0, 0.0, 50.0);
        let players = &self.players;
        let factory = self.window.factory.clone();
        let font_path = self.assets.join("Roboto-Regular.ttf");

        self.window.draw_2d(event, |c: Context, gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
            let mut cache = piston_window::Glyphs::new(
                font_path,
                factory).unwrap();
            text(WHITE, 14, &fps_text, &mut cache, transform, gl);

            for player in players {
                let transform = c.transform.trans(player.position.x, player.position.y)
                                            .rot_rad(player.rotation)
                                            .trans(-square[2] * 0.5, -square[3] * 0.5)
                                            .scale(0.5, 0.5);

                // Set our player sprite position.
                image(&player.tex, transform, gl);

                // Draw our projectiles.
                for projectile in &player.projectiles {
                    let square = rectangle::square(0.0, 0.0, 5.0);
                    let transform = c.transform.trans(projectile.position.x, projectile.position.y)
                        .rot_rad(projectile.rotation)
                        .trans(-square[2] * 0.5, -square[3] * 0.5);
                    image(player.projectile_texture.deref(), transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Update our players.
        for player in &mut self.players {
            player.update(args);
        }
    }
}

fn apply_input(players:&mut Vec<Player>, key_states: &HashMap<Key, input::KeyState>, dt: f64) {
    let mut player_velocities: Vec<Vector2> = vec![
        Vector2::default(),
        Vector2::default()
    ];

    for (key, value) in key_states {
        match *key {
            // Player 1
            Key::W => {
                if value.pressed || value.held {
                    player_velocities[0].y -= 1.0 * dt;
                }
            },
            Key::A => {
                if value.pressed || value.held {
                    player_velocities[0].x -= 1.0 * dt;
                }
            },
            Key::S => {
                if value.pressed || value.held {
                    player_velocities[0].y += 1.0 * dt;
                }
            },
            Key::D => {
                if value.pressed || value.held {
                    player_velocities[0].x += 1.0 * dt;
                }
            },

            // Player 2
            Key::Up => {
                if value.pressed || value.held {
                    player_velocities[1].y -= 1.0 * dt;
                }
            },
            Key::Left => {
                if value.pressed || value.held {
                    player_velocities[1].x -= 1.0 * dt;
                }
            },
            Key::Down => {
                if value.pressed || value.held {
                    player_velocities[1].y += 1.0 * dt;
                }
            },
            Key::Right => {
                if value.pressed || value.held {
                    player_velocities[1].x += 1.0 * dt;
                }
            },
            // Player1
            Key::Space => {
                if value.pressed {
                    players[0].shoot();
                }
            },
            // Player 2
            Key::Return => {
                if value.pressed {                    
                    players[1].shoot();
                }
            },
            // Default
            _ => {}
        }
    }

    for i in 0..players.len() {
        if player_velocities[i] == Vector2::default() {
            continue
        }
        player_velocities[i].normalize();
        players[i].position += player_velocities[i] * MOVE_SPEED_MAX * dt;
    }
}

fn main() {
    let width = 800;
    let height = 800;

    let window_settings = WindowSettings::new(
        "piston_shooty",
        [width, height]);

    let mut assets: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let mut window : piston_window::PistonWindow = window_settings
        .exit_on_esc(true)
        .build()
        .unwrap();

    let asset_loader = AssetLoader { };
    let hand_gun_red = asset_loader.load_texture("hand-gun-red.png", &mut window.factory, &mut assets);
    let hand_gun_blue = asset_loader.load_texture("hand-gun-blue.png", &mut window.factory, &mut assets);
    let gun_gun = Rc::new(asset_loader.load_texture("GunGunV1.png", &mut window.factory, &mut assets));

    let mut app = App {
        window: window,
        players: vec![
            Player {
                // team: TEAM1,
                position: Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                projectiles: Vec::new(),
                tex: hand_gun_blue,
                projectile_texture: gun_gun.clone(),
            },
            Player {
                // team: TEAM2,
                position: Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                projectiles: Vec::new(),
                tex: hand_gun_red,
                projectile_texture: gun_gun.clone(),
            },
        ],
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        assets: assets
    };
    app.window.set_max_fps(u64::max_value());

    let mut key_states: HashMap<Key, input::KeyState> = HashMap::new();
    
    while let Some(e) = app.window.next() {
        // Update.
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        
        input::gather_input(&e, &mut key_states);

        if let Some(u) = e.update_args() {
            apply_input(&mut app.players, &key_states, u.dt);
            input::update_input(&mut key_states)
        }

        // Render.
        if e.render_args().is_some() {
            app.render(&e);
        }
    }
}

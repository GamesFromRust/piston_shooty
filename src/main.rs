mod input;
mod vector2;

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate time;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate sprite;
extern crate graphics;

use sprite::*;
use std::collections::HashMap;
use piston_window::*;
use vector2::*;
use std::rc::Rc;
use graphics::ImageSize;

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 300.0;
const PLAYER_ROTATIONAL_VELOCITY: f64 = 3.0;
const GREEN:    [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE:    [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MOVE_SPEED_MAX: f64 = 100.0;

const NSEC_PER_SEC: u64 = 1_000_000_000;

pub struct Team {
    team_color: [f32; 4]
}

const NO_TEAM: Team = Team {team_color: GREEN};
const TEAM1: Team = Team {team_color: BLUE};
const TEAM2: Team = Team {team_color: RED};

pub struct Projectile {
    position: Vector2,
    velocity: Vector2,
}

pub struct Player {
    team: Team,
    position: Vector2,
    rotation: f64,
    projectiles: Vec<Projectile>,
}

impl Default for Player {
  fn default () -> Player {
    Player {
        team: NO_TEAM,
        position: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        projectiles: Vec::new(),
    }
  }
}

impl Player {
    fn shoot(&mut self) {
        let x = self.rotation.cos();
        let y = self.rotation.sin();
        let mut vel = Vector2 {x: x, y: y};
        vel *= PROJECTILE_VELOCITY_MAGNITUDE;

        let projectile = Projectile {
            position: self.position,
            velocity: vel
        };

        self.projectiles.push(projectile);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += PLAYER_ROTATIONAL_VELOCITY * args.dt;
        // Move our projectiles.
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * args.dt;
        }
    }
}

pub struct App {
    window: piston_window::PistonWindow,
    players: Vec<Player>,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64
}

impl App {
    fn render(&mut self, event: &Event, scene: &Scene<ImageSize>) {
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

        self.window.draw_2d(event, |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw our fps.
            let transform = c.transform.trans(10.0, 10.0);
                let mut cache = piston_window::Glyphs::new(
                    "D:\\Development\\Rust\\piston_shooty\\assets\\Roboto-Regular.ttf", // TODO: Change at some point.
                    factory).unwrap();
                text(WHITE, 14, &fps_text, &mut cache, transform, gl);

            
            for player in players {
                let transform = c.transform.trans(player.position.x, player.position.y)
                                            .rot_rad(player.rotation)
                                            .trans(-square[2] * 0.5, -square[3] * 0.5);
                
                // Draw a box rotating around the middle of the screen.
                rectangle(player.team.team_color, square, transform, gl);

                // Draw our projectiles.
                for projectile in &player.projectiles {
                    let square = rectangle::square(0.0, 0.0, 5.0);
                    let transform = c.transform.trans(projectile.position.x, projectile.position.y)
                        .trans(-square[2] * 0.5, -square[3] * 0.5);
                    rectangle(player.team.team_color, square, transform, gl);
                }
            }

            scene.draw(c.transform, gl);
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
                if value.pressed {
                    player_velocities[0].y -= 1.0;
                }
            },
            Key::A => {
                if value.pressed {
                    player_velocities[0].x -= 1.0;
                }
            },
            Key::S => {
                if value.pressed {
                    player_velocities[0].y += 1.0;
                }
            },
            Key::D => {
                if value.pressed {
                    player_velocities[0].x += 1.0;
                }
            },

            // Player 2
            Key::Up => {
                if value.pressed {
                    player_velocities[1].y -= 1.0;
                }
            },
            Key::Left => {
                if value.pressed {
                    player_velocities[1].x -= 1.0;
                }
            },
            Key::Down => {
                if value.pressed {
                    player_velocities[1].y += 1.0;
                }
            },
            Key::Right => {
                if value.pressed {
                    player_velocities[1].x += 1.0;
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
    let window = WindowSettings::new(
                "piston_shooty",
                [width, height]
            )
            .exit_on_esc(true)
            .build()
            .unwrap();

    let scene: Scene = Scene::new();

    let tex = Rc::new(
        Texture::from_path(
            &mut window.factory,
            "assets/hand-gun.png"
            ),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let mut sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64/ 2.0, height as f64 / 2.0);

    scene.add_child(sprite);

    let mut app = App {
        window: window,
        players: vec![
            Player {team: TEAM1, sprite: sprite.clone(), ..Default::default()},
            Player {team: TEAM2, sprite: sprite.clone(), ..Default::default()},
        ],
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1
    };
    app.window.set_max_fps(u64::max_value());

    let mut key_states: HashMap<Key, input::KeyState> = HashMap::new();
    
    while let Some(e) = app.window.next() {
        // // Render.
        if e.render_args().is_some() {
            app.render(&e, &scene);
        }

        // Update.
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        
        input::gather_input(&e, &mut key_states);

        if let Some(u) = e.update_args() {
            apply_input(&mut app.players, &key_states, u.dt);
        }
    }
}

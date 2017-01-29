mod input;

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate time;
// extern crate find_folder;
extern crate piston_window;
// extern crate opengl_graphics;
extern crate gfx_device_gl;

use opengl_graphics::glyph_cache::GlyphCache;
//use piston::window;
//use piston::window::WindowSettings;
//use piston::event_loop::*;
// use piston::event_loop::*;
// use piston::event_loop::WindowEvents;
//use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics/*, OpenGL*/ };
use std::collections::HashMap;
use std::ops::*;
use std::rc::Rc;
use piston_window::*;

#[derive(Clone, Copy)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

const PROJECTILE_VELOCITY_MAGNITUDE: f64 = 300.0;
const PLAYER_ROTATIONAL_VELOCITY: f64 = 3.0;
const GREEN:    [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE:    [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const NSEC_PER_SEC: u64 = 1_000_000_000;

pub struct Team {
    team_color: [f32; 4]
}

const NO_TEAM: Team = Team {team_color: GREEN};
const TEAM1: Team = Team {team_color: BLUE};
const TEAM2: Team = Team {team_color: RED};

impl Vector2 {
    fn normalize(&mut self) {
        let mag = ((self.x * self.x) + (self.y * self.y) as f64).sqrt();
        self.x /= mag;
        self.y /= mag;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// TODO: How the fuck does the memory work out here? What's copied?
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub struct Projectile {
    position: Vector2,
    velocity: Vector2,
}

pub struct Player {
    team: Team,
    position: Vector2,
    rotation: f64,
    projectiles: Vec<Projectile>
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
    //gl: GlGraphics, // OpenGL drawing backend.
    window: piston_window::PistonWindow,
    players: Vec<Player>,
    // last_frame_time: u64,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    // total_batch_time: u64,
    average_frame_time: u64
}

impl App {
    fn render(&mut self, event: &Event) {
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

                // let text = Text::new(14);
                // let mut cache = character::CharacterCache::new();
                // text.draw(fps_text, &mut cache, &c.draw_state, transform, gl)

                // text.draw(&e, |c, g| {
                //     let transform = c.transform.trans(10.0, 100.0);
                //     let mut glyphs = Glyphs::new();
                //     // Set a white background
                //     clear([1.0, 1.0, 1.0, 1.0], g);
                //     text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                //         &fps_text,
                //         &mut glyphs,
                //         &c.draw_state,
                //         transform, g
                //     );
                // });
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

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    //let opengl = OpenGL::V3_2;

    // Create a new game and run it.
    let mut app = App {
        //gl: GlGraphics::new(opengl),
        window: WindowSettings::new(
                "piston_shooty",
                [800, 800]
            )
            //.opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(),
        // piston_window::PistonWindow::build_from_window_settings(WindowSettings::new(
        //         "piston_shooty",
        //         [800, 800]
        //     )
        //     .opengl(opengl)
        //     .exit_on_esc(true))
        //     .unwrap(),
        players: vec![
            Player {team: TEAM1, ..Default::default()},
            Player {team: TEAM2, ..Default::default()},
        ],
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1
    };

    // let mut events: piston::event_loop::WindowEvents = app.window.events();
    let mut key_states: HashMap<Key, input::KeyState> = HashMap::new();
    
    while let Some(e) = app.window.next() {
        //app.render(&e);
        //app.update(&e);
        // // Render.
        if let Some(r) = e.render_args() {
            app.render(&e);
        }

        // Update.
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        
        input::gather_input(&e, &mut key_states);

        // TODO: Change to not trigger literally every generic event.
        for (key, value) in &key_states {
            match *key {
                // Player 1
                Key::W => {
                    if value.pressed {
                        app.players[0].position.y -= 1.0;
                    }
                },
                Key::A => {
                    if value.pressed {
                        app.players[0].position.x -= 1.0;
                    }
                },
                Key::S => {
                    if value.pressed {
                        app.players[0].position.y += 1.0;
                    }
                },
                Key::D => {
                    if value.pressed {
                        app.players[0].position.x += 1.0;
                    }
                },

                // Player 2
                Key::Up => {
                    if value.pressed {
                        app.players[1].position.y -= 1.0;
                    }
                },
                Key::Left => {
                    if value.pressed {
                        app.players[1].position.x -= 1.0;
                    }
                },
                Key::Down => {
                    if value.pressed {
                        app.players[1].position.y += 1.0;
                    }
                },
                Key::Right => {
                    if value.pressed {
                        app.players[1].position.x += 1.0;
                    }
                },
                // Player1
                Key::Space => {
                    if value.pressed {
                        app.players[0].shoot();
                    }
                },
                // Player 2
                Key::Return => {
                    if value.pressed {                    
                        app.players[1].shoot();
                    }
                },

                // Default
                _ => {}
            }
        }
    }
}

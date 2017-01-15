extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::HashMap;
use std::ops::*;

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
    gl: GlGraphics, // OpenGL drawing backend.
    players: Vec<Player>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let players = &self.players;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            
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
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Update our players.
        for player in &mut self.players {
            player.update(args);
        }
    }
}

pub struct KeyState {
    held: bool,
    pressed: bool,
    released: bool,
}

fn gather_input(e: piston::event_loop::WindowEvents, key_states:&mut HashMap<Key, KeyState>) -> HashMap<Key, KeyState> {    
    if let Some(Button::Keyboard(key)) = e.press_args() {
        let key_state = match key_states.get(key) {
            Some(state) => {
                if state.pressed {
                    state.held = true;
                }
                state
            },
            None => KeyState { held: false, pressed: true, released: false }
        };

        key_states.insert(key, key_state);
    }
    if let Some(Button::Keyboard(key)) = e.release_args() {
        let key_state = key_states.get(key);
        key_state.pressed = false;
        key_state.held = false;
        key_states.insert(key, false);
    }
    key_states
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
            "piston_shooty",
            [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        players: vec![
            Player {team: TEAM1, ..Default::default()},
            Player {team: TEAM2, ..Default::default()},
        ]
    };

    let mut events = window.events();
    let mut key_states = HashMap::new();
    
    while let Some(e) = events.next(&mut window) {
        // Render.
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // Update.
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        let () = e;

        // Keyboard.
        key_states = gather_input(e, &mut key_states);

        // Move our shit.
        for (key, value) in &key_states {
            match *key {
                // Player 1
                Key::W => {
                    if *value.pressed {
                        app.players[0].position.y -= 1.0;
                    }
                },
                Key::A => {
                    if *value.pressed {
                        app.players[0].position.x -= 1.0;
                    }
                },
                Key::S => {
                    if *value.pressed {
                        app.players[0].position.y += 1.0;
                    }
                },
                Key::D => {
                    if *value.pressed {
                        app.players[0].position.x += 1.0;
                    }
                },

                // Player 2
                Key::Up => {
                    if *value.pressed {
                        app.players[1].position.y -= 1.0;
                    }
                },
                Key::Left => {
                    if *value.pressed {
                        app.players[1].position.x -= 1.0;
                    }
                },
                Key::Down => {
                    if *value.pressed {
                        app.players[1].position.y += 1.0;
                    }
                },
                Key::Right => {
                    if *value.pressed {
                        app.players[1].position.x += 1.0;
                    }
                },
                // Player1
                Key::Space => {
                    if *value.pressed {
                        app.players[0].shoot();
                    }
                },
                // Player 2
                Key::Return => {
                    if *value.pressed {                    
                        app.players[1].shoot();
                    }
                },

                // Default
                _ => {}
            }
        }
    }
}

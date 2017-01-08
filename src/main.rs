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
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(&mut self) {
        let mut mag = ((self.x * self.x) + (self.y * self.y) as f64).sqrt();
        self.x /= mag;
        self.y /= mag;
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// TODO: How the fuck does the memory work out here? What's copied?
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

pub struct Projectile {
    position: Point,
    velocity: Point,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    position: Point,
    projectiles: Vec<Projectile>
}

impl App {
    fn shoot(&mut self, cursor:&Point) {
        let mut vel = *cursor - self.position;
        vel.normalize();

        let mut projectile = Projectile {
            position: self.position,
            velocity: vel
        };

        self.projectiles.push(projectile);
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN:    [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let position = self.position;
        let projectiles = &self.projectiles;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(position.x, position.y)
                                        .rot_rad(rotation)
                                        .trans(-square[2] * 0.5, -square[3] * 0.5);
            
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            // Draw our projectiles.
            for projectile in projectiles {
                let square = rectangle::square(0.0, 0.0, 5.0);
                let transform = c.transform.trans(projectile.position.x, projectile.position.y)
                    .trans(-square[2] * 0.5, -square[3] * 0.5);
                rectangle(BLUE, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        // Move our projectiles.
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity;
        }
    }
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
        rotation: 0.0,
        position: Point {x: 0.0, y: 0.0},
        projectiles: Vec::new()
    };

    let mut key_state = HashMap::new();
    let mut cursor = Point { 
                                x: 0.0, 
                                y: 0.0 
                            };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        // Render.
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // Update.
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        e.mouse_cursor(|x, y| {
            cursor = Point { x: x, y:y };
            println!("Mouse moved '{} {}'", x, y);
        });

        // Mouse.
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                app.shoot(&cursor);
            }
        }

        // Keyboard.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            key_state.insert(key, true);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            key_state.insert(key, false);
        }

        // Move our shit.
        for (key, value) in &key_state {
            match *key {
                Key::W => {
                    if *value {
                        app.position.y -= 1.0;
                    }
                },
                Key::A => {
                    if *value {
                        app.position.x -= 1.0;
                    }
                },
                Key::S => {
                    if *value {
                        app.position.y += 1.0;
                    }
                },
                Key::D => {
                    if *value {
                        app.position.x += 1.0;
                    }
                },
                _ => {}
            }
        }
    }
}

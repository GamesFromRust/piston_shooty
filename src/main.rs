mod input;
mod vector2;
mod asset_loader;
mod texture_manager;
mod font_manager;
mod sound_manager;
mod world;
mod renderable_object;
mod renderable;
mod object_type;
mod updatable;
mod ground;
mod wall;
mod enemy;
mod bullet;
mod player;
mod game_state;
mod render_utils;
mod victory_screen;
mod game_state_utils;
mod menu_screen;
mod colors;
mod collidable;
mod hand_gun;
mod collidable_object;
mod game_object;
mod gun_strategy;
mod gun;
mod gun_axe;

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
use texture_manager::TextureManager;
use sound_manager::SoundManager;
use font_manager::FontManager;
use std::ops::DerefMut;
use std::io::{self, Write};
use csv::index::{Indexed, create_index};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use world::World;
use player::Player;
use wall::Wall;
use ground::Ground;
use enemy::Enemy;
use world::GameEndedState;
use renderable_object::RenderableObject;
use game_state::GameState;
use game_state::GameStateType;
use game_state::UpdateResult;
use game_state::UpdateResultType;
use victory_screen::VictoryScreen;
use menu_screen::MenuScreen;
use collidable_object::CollidableObject;
use gun_axe::GunAxe;
use hand_gun::HandGun;
use gun::Gun;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const BLUE:     [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const NSEC_PER_SEC: u64 = 1_000_000_000;
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

pub struct App<'a> {
    window: piston_window::PistonWindow,
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    font_manager: FontManager,
    window_height: f64,
    window_width: f64,
    game_state: Box<GameState>,
    texture_manager: TextureManager,
    sound_manager: SoundManager,
    level_index: usize,
    world_list: Rc<Vec<&'a str>>
}

impl<'a> App<'a> {
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

        let font_manager = &mut self.font_manager;
        let window_width = self.window_width;
        let window_height = self.window_height;
        // let game_ended_state = &self.world.game_ended_state;
        let game_state = &self.game_state;

        self.window.draw_2d(event, |c: Context, gl: &mut G2d| {
            // Clear the screen.
            clear(GREEN, gl);

            game_state.render(c, gl, font_manager, window_width, window_height);

            // Draw our fps.
            let fps_transform = c.transform.trans(10.0, 10.0);
            let cache_rc = font_manager.get("Roboto-Regular.ttf");
            text(WHITE, 14, &fps_text, cache_rc.borrow_mut().deref_mut(), fps_transform, gl);
        });
    }

    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) {
        let update_result = self.game_state.update(&key_states, &mouse_states, &mouse_pos, &args);
        self.advance_game_state(update_result);
    }

    fn advance_game_state_from_world_select(&mut self, update_result:UpdateResult) {
        match update_result.result_type {
            UpdateResultType::Running => {
                // do nothing
            },
            UpdateResultType::Success => {
                self.level_index = update_result.result_code as usize;
                let world = load_level(&mut self.texture_manager, &mut self.sound_manager, self.world_list[self.level_index]);
                self.game_state = Box::new(world);
            },
            UpdateResultType::Fail => {
                // do nothing
            },
        }
    }

    fn advance_game_state_from_world(&mut self, update_result:UpdateResult) {
        match update_result.result_type {
            UpdateResultType::Running => {
                // do nothing
            },
            UpdateResultType::Success => {
                self.level_index = self.level_index + 1;
                self.advance_level();
            },
            UpdateResultType::Fail => {
                self.advance_level();
            },
        }
    }

    fn advance_game_state(&mut self, update_result:UpdateResult) {
        if GameStateType::WorldSelect == self.game_state.get_type() {
            self.advance_game_state_from_world_select(update_result);
        } else if GameStateType::World == self.game_state.get_type() {
            self.advance_game_state_from_world(update_result);
        } else if GameStateType::Victory == self.game_state.get_type() {
            self.advance_game_state_from_world(update_result);
        }
    }

    fn advance_level(&mut self) {
        if self.level_index > self.world_list.len() {
            self.level_index = 0;
        }

        if self.level_index < self.world_list.len() {
            let world = load_level(&mut self.texture_manager, &mut self.sound_manager, self.world_list[self.level_index]);
            self.game_state = Box::new(world);
        } else if self.level_index == self.world_list.len() {
            self.game_state = Box::new(VictoryScreen{});
        }
    }
}

fn load_level(texture_manager:&mut TextureManager, sound_manager:&mut SoundManager, level_name:&str) -> World {
    let hand_gun_texture = texture_manager.get("textures\\hand-gun_square.png");
    let axe_gun_texture = texture_manager.get("textures\\GunaxeV1.png");
    let gun_gun = texture_manager.get("textures\\GunGunV1.png");
    let bullet = texture_manager.get("textures\\bullet.png");
    let wall = texture_manager.get("textures\\brick_square.png");
    let enemy = texture_manager.get("textures\\enemy.png");
    let ground = texture_manager.get("textures\\ground.png");
    
    let gun_sound = sound_manager.get("sounds\\boom.ogg");

    let hand_gun: Rc<Gun> = Rc::new( Gun {
        position: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        rotation: 0.0,
        scale: 1.0,
        renderable_object: RenderableObject {
            texture: gun_gun.clone(),
        },
        velocity: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        collidable_object: CollidableObject {
            width: gun_gun.get_size().0 as f64,
            height: gun_gun.get_size().1 as f64,
        },
        gun_sound: gun_sound.clone(),
        gun_texture: gun_gun.clone(),
        gun_strategy: Box::new(HandGun {
            should_delete: false
        })
    });

    let gun_axe: Rc<Gun> = Rc::new( Gun {
        position: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        rotation: 0.0,
        scale: 1.0,
        renderable_object: RenderableObject {
            texture: axe_gun_texture.clone(),
        },
        velocity: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        collidable_object: CollidableObject {
            width: axe_gun_texture.get_size().0 as f64,
            height: axe_gun_texture.get_size().1 as f64,
        },
        gun_sound: gun_sound.clone(),
        gun_texture: axe_gun_texture.clone(),
        gun_strategy: Box::new(GunAxe {
            should_delete: false
        })
    });

    let guns: Vec<Rc<Gun>> = vec![
        hand_gun.clone(),
        gun_axe.clone(),
    ];

    let player: Player = Player {
        position: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        rotation: 0.0,
        scale: PLAYER_SCALE,
        renderable_object: RenderableObject {
            texture: hand_gun_texture.clone(),
        },
        guns: Vec::new(),
        bullet_texture: bullet.clone(),
        bullet_sound: sound_manager.get("sounds\\boop.ogg"),
        has_shot_bullet: false,
        gun_template: hand_gun.clone(),
        gun_templates: guns,
        current_gun_template_index: 0,
    };
    
    let player = Rc::new(RefCell::new(player));
    
    let (sender, receiver) = channel();

    let mut world: World = World {
        renderables: Vec::new(),
        collidables: Vec::new(),
        updatables: Vec::new(),
        game_ended_state: GameEndedState {
            game_ended: false,
            won: false
        },
        player: player.clone(),
        receiver: receiver,
        should_display_level_name: true,
        name: String::from(level_name),
    };

    let new_csv_rdr = || csv::Reader::from_file(format!("assets\\Levels\\{}.csv", level_name)).unwrap().has_headers(false);
    let mut index_data = io::Cursor::new(Vec::new());
    create_index(new_csv_rdr(), index_data.by_ref()).unwrap();
    let mut index = Indexed::open(new_csv_rdr(), index_data).unwrap();

    let mut level: Vec<Vec<String>> = Vec::new();
    for row in index.records() {
        let row = row.unwrap();
        
        // for item in &row {
        //     print!("{},", item);
        // }
        // println!("");

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
                    position: Vector2 {
                        x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                        y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                    },
                    rotation: 0.0,
                    scale: WALL_SCALE,
                    renderable_object: RenderableObject {
                        texture: wall.clone(),
                    },
                    collidable_object: CollidableObject {
                        width: wall.get_size().0 as f64,
                        height: wall.get_size().1 as f64,
                    },
                };
                let refcell = Rc::new(RefCell::new(wall));
                world.add_renderable_at_layer(refcell.clone(), WALL_LAYER);
                world.add_collidable(refcell.clone());
            } else if item == "P" {
                let ground = Ground {
                    position: Vector2 {
                        x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                        y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                    },
                    rotation: 0.0,
                    scale: GROUND_SCALE,
                    renderable_object: RenderableObject {
                        texture: ground.clone(),
                    },
                };
                let refcell = Rc::new(RefCell::new(ground));
                world.add_renderable_at_layer(refcell.clone(), GROUND_LAYER);

                player.borrow_mut().position.x = (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64;
                player.borrow_mut().position.y = (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64;

                world.add_renderable_at_layer(player.clone(), PLAYER_LAYER);
                world.add_updatable(player.clone());
            } else if item == "E" {
                let ground = Ground {
                    position: Vector2 {
                        x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                        y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                    },
                    rotation: 0.0,
                    scale: GROUND_SCALE,
                    renderable_object: RenderableObject {
                        texture: ground.clone(),
                    },
                };
                let refcell = Rc::new(RefCell::new(ground));
                world.add_renderable_at_layer(refcell.clone(), GROUND_LAYER);

                let enemy = Enemy {
                    position: Vector2 {
                        x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64,
                        y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                    },
                    rotation: 0.0,
                    scale: ENEMY_SCALE,
                    renderable_object: RenderableObject {
                        texture: enemy.clone(),
                    },
                    should_delete: false,
                    collidable_object: CollidableObject {
                        width: enemy.get_size().0 as f64,
                        height: enemy.get_size().1 as f64,
                    },
                };
                let refcell = Rc::new(RefCell::new(enemy));
                world.add_renderable_at_layer(refcell.clone(), ENEMY_LAYER);
                world.add_collidable(refcell.clone());
            } else if item == "_" {
                // todo: make this a func and factor out from 3 ifs above
                let ground = Ground {
                    position: Vector2 {
                        x: (item_num * CELL_WIDTH + CELL_WIDTH / 2) as f64 ,
                        y: (line_num * CELL_HEIGHT + CELL_HEIGHT / 2) as f64
                    },
                    rotation: 0.0,
                    scale: GROUND_SCALE,
                    renderable_object: RenderableObject {
                        texture: ground.clone(),
                    },
                };
                let refcell = Rc::new(RefCell::new(ground));
                world.add_renderable_at_layer(refcell.clone(), GROUND_LAYER);
            }
            item_num += 1;
        }
        line_num += 1;
    }

    // Spawn one second timer.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        // The send only fails if the receiver is disconnected.
        // For us, this (probably) means the receiver's been deallocated
        // and replaced with the next world's receiver.
        let _ = sender.send(0);
    });

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
    
    let texture_manager = TextureManager {
        asset_loader: asset_loader.clone(),
        textures_by_filename: HashMap::new(),
    };

    let sound_manager = SoundManager {
        asset_loader: asset_loader.clone(),
        sounds_by_filename: HashMap::new(),
    };

    font_manager.get("Roboto-Regular.ttf");
    
    let world_list = Rc::new(vec!["Sunday-Gunday", "Multi-Level Mark-hitting"]);

    let menu_screen = MenuScreen {
        world_list: world_list.clone(),
        selected_world_index: 0,
    };
    
    let mut app = App {
        window: window,
        last_batch_start_time: time::precise_time_ns(),
        num_frames_in_batch: 0,
        average_frame_time: 1,
        font_manager: font_manager,
        window_height: HEIGHT as f64,
        window_width: WIDTH as f64,
        game_state: Box::new(menu_screen),
        texture_manager: texture_manager,
        sound_manager: sound_manager,
        level_index: 0,
        world_list: world_list
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
            app.update(&key_states, &mouse_states, &mouse_pos, &u);
            input::update_input(&mut key_states, &mut mouse_states);
        }

        // Render.
        if e.render_args().is_some() {
            app.render(&e);
        }
    }
}
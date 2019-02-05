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
mod collidable;
mod hand_gun;
mod collidable_object;
mod game_object;
mod gun_strategy;
mod gun;
mod gun_axe;
mod meta_gun;
mod ui_bundle;
mod ui_widget_ids;
mod fps_counter;

extern crate piston;
extern crate glutin_window;
extern crate time;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate graphics;
extern crate find_folder;
extern crate ears;
extern crate ncollide2d;
extern crate nalgebra;
extern crate csv;
#[macro_use] extern crate conrod_core;
extern crate conrod_piston;

use std::collections::HashMap;
use piston_window::*;
use crate::vector2::*;
use crate::asset_loader::AssetLoader;
use std::rc::Rc;
use std::cell::RefCell;
use crate::texture_manager::TextureManager;
use crate::sound_manager::SoundManager;
use crate::font_manager::FontManager;
use std::fs::File;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use crate::world::World;
use crate::player::Player;
use crate::wall::Wall;
use crate::ground::Ground;
use crate::enemy::Enemy;
use crate::world::GameEndedState;
use crate::renderable_object::RenderableObject;
use crate::game_state::GameState;
use crate::game_state::GameStateType;
use crate::game_state::UpdateResult;
use crate::game_state::UpdateResultType;
use crate::victory_screen::VictoryScreen;
use crate::menu_screen::MenuScreen;
use crate::collidable_object::CollidableObject;
use crate::gun_axe::GunAxe;
use crate::hand_gun::HandGun;
use crate::meta_gun::MetaGun;
use crate::ui_bundle::UiBundle;
use crate::ui_widget_ids::Ids;
use crate::fps_counter::FpsCounter;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
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
    game_state: Box<GameState>,
    texture_manager: TextureManager,
    sound_manager: SoundManager,
    level_index: usize,
    world_list: Rc<Vec<&'a str>>,
    ui_bundle: UiBundle<'a>,
}

impl<'a> App<'a> {
    fn render(&mut self, event: &Event) {
        let game_state = &mut self.game_state;
        let ui_bundle = &mut self.ui_bundle;

        self.window.draw_2d(event, |c: graphics::Context, gl/*: &mut G2d*/| {
            clear(GREEN, gl);

            game_state.render(c, gl, ui_bundle);
        });
    }

    fn update(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) {
        let update_result = self.game_state.update(&key_states, &mouse_states, &mouse_pos, &mut self.ui_bundle, &args);
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
            self.game_state = Box::new(VictoryScreen{
                image_map: conrod_core::image::Map::new()
            });
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

    let hand_gun: RefCell<MetaGun> = RefCell::new( MetaGun {
        gun_sound: gun_sound.clone(),
        gun_texture: gun_gun.clone(),
        bullet_texture: bullet.clone(),
        bullet_sound: sound_manager.get("sounds\\boop.ogg"),
        gun_strategy: Box::new(HandGun {
            should_delete: false
        }),
        shots_taken: 0,
        guns: Vec::new(),
        has_shot_bullet: false,
    });

    let gun_axe: RefCell<MetaGun> = RefCell::new( MetaGun {
        gun_sound: gun_sound.clone(),
        gun_texture: axe_gun_texture.clone(),
        bullet_texture: bullet.clone(),
        bullet_sound: sound_manager.get("sounds\\boop.ogg"),
        gun_strategy: Box::new(GunAxe {
            should_delete: false
        }),
        shots_taken: 0,
        guns: Vec::new(),
        has_shot_bullet: false,
    });

    let meta_guns: Vec<RefCell<MetaGun>> = vec![
        hand_gun,
        gun_axe,
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
        gun_templates: meta_guns,
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
        fps_counter: FpsCounter::default(),
        image_map: conrod_core::image::Map::new(),
    };

    let file_name = format!("assets\\Levels\\{}.csv", level_name);
    let file_result = File::open(file_name.clone());

    let file = match file_result {
        Ok(f) => f,
        Err(err) => {panic!("Couldn't read file from {}, err: {}", file_name, err);}
    };
//    let mut csv_rdr = csv::Reader::from_reader(file);
    let mut csv_rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    // Make sure it's the right size.
//    let mut i = 0;
//    for record_result in csv_rdr.records() {
//        let mut record = match record_result {
//            Ok(r) => r,
//            Err(err) => { panic!("Couldn't read line {} from {}", i, file_name); }
//        };
//
//        assert!(record.len() as u32 == GRID_WIDTH);
//        i += 1;
//    }
//    assert!(i == GRID_HEIGHT);

//    let new_csv_rdr = || csv::Reader::from_file(format!("assets\\Levels\\{}.csv", level_name)).unwrap().has_headers(false);
//    let mut index_data = io::Cursor::new(Vec::new());
//    create_index(new_csv_rdr(), index_data.by_ref()).unwrap();
//    let mut index = Indexed::open(new_csv_rdr(), index_data).unwrap();

    // Read in a level.
    let mut line_num = 0;
    for record_result in csv_rdr.records() {
        let line = match record_result {
            Ok(r) => r,
            Err(err) => {panic!("Couldn't read line {} from {}, err: {}", line_num, file_name, err);}
        };
        let mut item_num = 0;
        for item in line.iter() {
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

fn make_menu_screen<'a>(world_list: Rc<Vec<&'a str>>, asset_loader: &AssetLoader ) -> MenuScreen<'a> {
    let mut image_map = conrod_core::image::Map::new();

    let logo_texture: G2dTexture = asset_loader.load_texture("textures/GunGunV1.png");
    let logo_image_id = image_map.insert(logo_texture);

    MenuScreen {
        world_list: world_list,
        selected_world_index: 0,
        fps_counter: FpsCounter::default(),
        image_map: image_map,
        logo_image_id: logo_image_id,
    }
}

fn main() {
    let window_settings = WindowSettings::new("piston_shooty", [WIDTH, HEIGHT]);

    let assets_path: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut window: piston_window::PistonWindow = window_settings.exit_on_esc(true)
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

    let menu_screen = make_menu_screen(world_list.clone(), &asset_loader);

    let mut key_states: HashMap<Key, input::ButtonState> = HashMap::new();
    let mut mouse_states: HashMap<MouseButton, input::ButtonState> = HashMap::new();
    let mut mouse_pos = Vector2::default();
    
    // todo: dupes
    let assets_path: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .build();

    let font_path = assets_path.join("Roboto-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let (glyph_cache, text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;

        let cache = conrod_core::text::GlyphCache::builder().dimensions(WIDTH, HEIGHT).scale_tolerance(SCALE_TOLERANCE).position_tolerance(POSITION_TOLERANCE).build();

        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap();
        
        (cache, texture)
    };

    let ids = Ids::new(ui.widget_id_generator());
    
    let ui_bundle: UiBundle = UiBundle {
        conrod_ui: ui,
        glyph_cache: glyph_cache,
        text_texture_cache: text_texture_cache,
        ids: ids,
    };

    let mut app = App {
        window: window,
        game_state: Box::new(menu_screen),
        texture_manager: texture_manager,
        sound_manager: sound_manager,
        level_index: 0,
        world_list: world_list,
        ui_bundle: ui_bundle,
    };
    app.window.set_max_fps(u64::max_value());

    // TODO: Why is args.dt locked to 120fps for UpdateArgs?
    while let Some(event) = app.window.next() {
        // Convert the piston event to a conrod event.
        let size = app.window.size();
        let (win_w, win_h) = (size.width as conrod_core::Scalar, size.height as conrod_core::Scalar);
        if let Some(conrod_event) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            app.ui_bundle.conrod_ui.handle_event(conrod_event);
        }

        // Input.
        input::gather_input(&event, &mut key_states, &mut mouse_states, &mut mouse_pos);
        
        if let Some(u) = event.update_args() {
            app.update(&key_states, &mouse_states, &mouse_pos, &u);
            input::update_input(&mut key_states, &mut mouse_states);
        }

        // Render.
        if event.render_args().is_some() {
            app.render(&event);
        }
    }
}
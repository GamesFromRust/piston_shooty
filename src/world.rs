use std::collections::HashMap;
use piston_window::*;
use crate::vector2::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use ncollide2d::bounding_volume::BoundingVolume;
use ncollide2d::bounding_volume;
use ncollide2d::shape::Cuboid;
use crate::input;
use nalgebra;
use ncollide2d;
use crate::renderable::Renderable;
use crate::object_type::ObjectType;
use crate::updatable::Updatable;
use crate::player::Player;
use std::ops::Deref;
use crate::game_state::GameState;
use crate::game_state::GameStateType;
use crate::game_state::UpdateResult;
use crate::game_state::UPDATE_RESULT_SUCCESS;
use crate::game_state::UPDATE_RESULT_RUNNING;
use crate::game_state::UPDATE_RESULT_FAIL;
use crate::render_utils;
use crate::game_state_utils;
use crate::collidable::Collidable;
use crate::ui_bundle::UiBundle;
use conrod_core;
use conrod_core::color::Colorable;
use conrod_core::widget::Widget;
use crate::fps_counter::FpsCounter;
use conrod_core::position::Positionable;
use conrod_core::position::Sizeable;
use crate::gun::BULLET_SCALE;

const ENEMY_LAYER: usize = 1;
const PROJECTILE_LAYER: usize = 2;

pub struct GameEndedState {
    pub game_ended: bool,
    pub won: bool,
}

// TODO: Split DynamicRenderable into Updatable and Collidable
pub enum WorldRequestType {
    AddUpdatable,
    AddDynamicRenderable,
}

pub struct WorldReq {
    pub renderable: Option<Rc<RefCell<Renderable>>>,
    pub updatable: Option<Rc<RefCell<Updatable>>>,
    pub collidable: Option<Rc<RefCell<Collidable>>>,
    pub req_type: WorldRequestType,
}

// TODO: Add self/guns/bullets to here.
pub struct World {
    pub renderables: Vec<Vec<Rc<RefCell<Renderable>>>>,
    // doesn't need to be a refcell but how do we make it not???????
    pub collidables: Vec<Rc<RefCell<Collidable>>>,
    pub updatables: Vec<Rc<RefCell<Updatable>>>,
    pub game_ended_state: GameEndedState,
    pub player: Rc<RefCell<Player>>,
    pub receiver: Receiver<u64>,
    pub should_display_level_name: bool,
    pub name: String,
    pub fps_counter: FpsCounter,
    pub image_map: conrod_core::image::Map<G2dTexture>,
}

impl World {
    pub fn add_renderable_at_layer(&mut self, renderable: Rc<RefCell<Renderable>>, layer: usize) {
        while self.renderables.len() <= layer {
            self.renderables.push(Vec::new());
        }
        self.renderables[layer].push(renderable);
    }

    pub fn add_collidable(&mut self, collidable: Rc<RefCell<Collidable>>) {
        self.collidables.push(collidable);
    }

    pub fn add_updatable(&mut self, updatable: Rc<RefCell<Updatable>>) {
        self.updatables.push(updatable);
    }

    fn is_victorious(&self) -> bool {
        for renderable in &self.renderables[ENEMY_LAYER] {
            if renderable.borrow().get_object_type() == ObjectType::Enemy {
                return false;
            }
        }

        true
    }

    fn can_take_action(&self) -> bool {
        let mut can_take_action = self.player.borrow().can_shoot_gun();
        can_take_action = can_take_action || self.player.borrow().can_shoot_bullet();
        can_take_action
    }

    fn was_defeated(&self) -> bool {
        if self.can_take_action() {
            return false;
        }

        for renderable_layer in &self.renderables {
            for renderable in renderable_layer {
                if renderable.borrow().get_object_type() == ObjectType::Bullet || renderable.borrow().get_object_type() == ObjectType::GunAxe {
                    return false;
                }
            }
        }

        true
    }

    fn update_game_running(&mut self, key_states: &HashMap<Key, input::ButtonState>, mouse_states: &HashMap<MouseButton, input::ButtonState>, mouse_pos: &Vector2, args: &UpdateArgs) -> UpdateResult {
        let _ = self.receiver.try_recv().map(|_| self.should_display_level_name = false);

        if self.is_victorious() {
            self.game_ended_state = GameEndedState { game_ended: true, won: true };
            return UPDATE_RESULT_RUNNING;
        }

        if self.was_defeated() {
            self.game_ended_state = GameEndedState { game_ended: true, won: false };
            return UPDATE_RESULT_RUNNING;
        }

        for collidable1 in &self.collidables {
            for collidable2 in &self.collidables {
                if Rc::ptr_eq(collidable1, collidable2) {
                    continue;
                }

                if collides(collidable1.borrow().deref(), collidable2.borrow().deref()) {
                    collidable1.borrow_mut().collide(collidable2.borrow().get_object_type());
                    collidable2.borrow_mut().collide(collidable1.borrow().get_object_type());
                }
            }
        }

        for renderable_layer in &mut self.renderables {
            renderable_layer.retain(|ref renderable| {
                !renderable.borrow().get_should_delete()
            });
        }

        self.updatables.retain(|ref updatable| {
            !updatable.borrow().get_should_delete()
        });

        self.collidables.retain(|ref collidable| {
            !collidable.borrow().get_should_delete()
        });

        let mut world_reqs: Vec<WorldReq> = Vec::new();
        for updatable in &self.updatables {
            let current_world_reqs = &mut updatable.borrow_mut().update(&key_states, &mouse_states, &mouse_pos, &args);
            world_reqs.append(current_world_reqs);
        }

        for world_req in world_reqs {
            match world_req.req_type {
                WorldRequestType::AddDynamicRenderable => {
                    assert!(world_req.renderable.is_some());
                    assert!(world_req.collidable.is_some());
                    if let Some(renderable) = world_req.renderable {
                        self.add_renderable_at_layer(renderable, PROJECTILE_LAYER);
                    }
                    if let Some(collidable) = world_req.collidable {
                        self.add_collidable(collidable);
                    }
                }
                WorldRequestType::AddUpdatable => {
                    assert!(world_req.updatable.is_some());
                    if let Some(updatable) = world_req.updatable {
                        self.add_updatable(updatable);
                    }
                }
            }
        }

        UPDATE_RESULT_RUNNING
    }

    fn update_game_ended_lost(&self, mouse_states: &HashMap<MouseButton, input::ButtonState>) -> UpdateResult {
        if game_state_utils::did_click(&mouse_states) {
            return UPDATE_RESULT_FAIL;
        } else {
            return UPDATE_RESULT_RUNNING;
        }
    }

    fn update_game_ended_won(&self, mouse_states: &HashMap<MouseButton, input::ButtonState>) -> UpdateResult {
        if game_state_utils::did_click(&mouse_states) {
            return UPDATE_RESULT_SUCCESS;
        } else {
            return UPDATE_RESULT_RUNNING;
        }
    }

    // todo: gif of ctrl+f of shots_taken in our codebase
    fn update_ui(&self, ui_bundle: &mut UiBundle) {
        // TODO: Please help.
        let gun_templates = &self.player.borrow().gun_templates;
        ui_bundle.ids.guns_hud.resize(gun_templates.len(), &mut ui_bundle.conrod_ui.widget_id_generator());
        ui_bundle.ids.shots_taken_hud.resize(gun_templates.len(), &mut ui_bundle.conrod_ui.widget_id_generator());
        ui_bundle.ids.bullets_remaining_hud.resize(gun_templates.len(), &mut ui_bundle.conrod_ui.widget_id_generator());
        ui_bundle.ids.bullets_hud.resize(gun_templates.len(), &mut ui_bundle.conrod_ui.widget_id_generator());

        let mut ui_cell = ui_bundle.conrod_ui.set_widgets();
        conrod_core::widget::Canvas::new()
            .pad(40.0)
            .color(conrod_core::color::TRANSPARENT)
            .set(ui_bundle.ids.canvas, &mut ui_cell);

        if self.game_ended_state.game_ended {
            if self.game_ended_state.won {
                render_utils::draw_text_overlay("Success! Click to continue.", &mut ui_cell, &ui_bundle.ids, conrod_core::color::WHITE, 36);
            } else {
                render_utils::draw_text_overlay("Defeat! Click to retry.", &mut ui_cell, &ui_bundle.ids, conrod_core::color::WHITE, 36);
            }
        } else if self.should_display_level_name {
            render_utils::draw_text_overlay(self.name.as_str(), &mut ui_cell, &ui_bundle.ids, conrod_core::color::WHITE, 36);
        }

        let mut id_gun_right = ui_bundle.ids.canvas;
        let mut width_gun_right = 0.0;
        for i in 0..gun_templates.len() {
            // Draw gun texture & highlight selected
            let is_selected_gun = i == self.player.borrow().current_gun_template_index;
            let gun_image_id = if is_selected_gun {
                self.player.borrow().gun_templates[i].borrow().selected_gun_image_id
            } else {
                self.player.borrow().gun_templates[i].borrow().gun_image_id
            };

            // note: gun_texture is only used for the width and height right now
            let gun_texture = if is_selected_gun {
                self.player.borrow().gun_templates[i].borrow().selected_gun_texture.clone()
            } else {
                self.player.borrow().gun_templates[i].borrow().gun_texture.clone()
            };

            let mut gun_image = conrod_core::widget::Image::new(gun_image_id)
                .w_h(gun_texture.get_width() as f64, gun_texture.get_height() as f64);

            if id_gun_right == ui_bundle.ids.canvas {
                gun_image = gun_image.top_right_of(id_gun_right);
            } else {
                gun_image = gun_image.left_from(id_gun_right, width_gun_right);
            }
            gun_image.set(ui_bundle.ids.guns_hud[i], &mut ui_cell);

            let bullet_image_id = self.player.borrow().gun_templates[i].borrow().bullet_image_id;
            let bullet_texture = self.player.borrow().gun_templates[i].borrow().bullet_texture.clone();
            conrod_core::widget::Image::new(bullet_image_id)
                    .w_h((BULLET_SCALE  * 1.5) * bullet_texture.get_width() as f64, (BULLET_SCALE  * 1.5) * bullet_texture.get_height() as f64)
                    .down_from(ui_bundle.ids.guns_hud[i], 30.0)
                    .align_middle_x_of(ui_bundle.ids.guns_hud[i])
                    .set(ui_bundle.ids.bullets_hud[i], &mut ui_cell);

            // draw guns depth remaining
            let gun_depth_remaining_text = if self.player.borrow().gun_templates[i].borrow().has_gun_depth() {
                let shots_taken = self.player.borrow().gun_templates[i].borrow().shots_taken;
                let gun_depth = self.player.borrow().gun_templates[i].borrow().get_gun_depth();
                format!("{}/{}", gun_depth - shots_taken, gun_depth)
            } else {
                "∞".to_string()
            };

            conrod_core::widget::Text::new(gun_depth_remaining_text.as_str())
                .font_size(18)
                .color(conrod_core::color::WHITE)
                .right_from(ui_bundle.ids.guns_hud[i], 8.0)
                .set(ui_bundle.ids.shots_taken_hud[i], &mut ui_cell);

            // draw bullets remaining
            let bullets_remaining = if self.player.borrow().gun_templates[i].borrow().has_shot_bullet {
                0
            } else {
                1
            };

            let bullets_remaining_text = format!("{}/{}", bullets_remaining, 1);
            conrod_core::widget::Text::new(bullets_remaining_text.as_str())
                .font_size(18)
                .color(conrod_core::color::WHITE)
                .right_from(ui_bundle.ids.bullets_hud[i], 8.0)
                .set(ui_bundle.ids.bullets_remaining_hud[i], &mut ui_cell);

            id_gun_right = ui_bundle.ids.guns_hud[i];
            width_gun_right = gun_texture.get_width() as f64;
        }

        self.fps_counter.update_ui(&mut ui_cell, &ui_bundle.ids);
    }
}

impl GameState for World {
    fn render(&mut self, c: Context, mut gl: &mut G2d, ui_bundle: &mut UiBundle) {
        self.fps_counter.calculate_fps();

        for i in 0..self.renderables.len() {
            for renderable in &self.renderables[i] {
                render_renderable(&c, &mut gl, renderable.borrow().deref());
            }
        }

        ui_bundle.render_ui(c, gl, &self.image_map);
    }

    fn update(
        &mut self,
        key_states: &HashMap<Key, input::ButtonState>,
        mouse_states: &HashMap<MouseButton, input::ButtonState>,
        mouse_pos: &Vector2,
        ui_bundle: &mut UiBundle,
        args: &UpdateArgs) -> UpdateResult {
        self.update_ui(ui_bundle);

        if self.game_ended_state.game_ended == false && self.game_ended_state.won == false {
            return self.update_game_running(&key_states, &mouse_states, &mouse_pos, &args);
        }

        if self.game_ended_state.game_ended == true && self.game_ended_state.won == false {
            return self.update_game_ended_lost(&mouse_states);
        }

        if self.game_ended_state.game_ended == true && self.game_ended_state.won == true {
            return self.update_game_ended_won(&mouse_states);
        }

        assert_eq!(false, true, "Invalid game ended state! Shouldn't have gotten here!");
        UPDATE_RESULT_RUNNING
    }

    fn get_type(&self) -> GameStateType {
        return GameStateType::World;
    }
}

fn collides(collidable1: &Collidable, collidable2: &Collidable) -> bool {
    let collidable1_aabb_cuboid2 = create_aabb_cuboid2(collidable1);
    let collidable2_aabb_cuboid2 = create_aabb_cuboid2(collidable2);
    collidable1_aabb_cuboid2.intersects(&collidable2_aabb_cuboid2)
}

fn create_aabb_cuboid2(collidable: &Collidable) -> ncollide2d::bounding_volume::aabb::AABB<f64> {
    let half_extents: nalgebra::core::Vector2<f64> = nalgebra::core::Vector2::new(
        collidable.get_collidable_object().width as f64 * 0.5 * collidable.get_scale(),
        collidable.get_collidable_object().height as f64 * 0.5 * collidable.get_scale());
    let cuboid2 = Cuboid::new(half_extents);
    let cuboid2_pos = nalgebra::geometry::Isometry2::new(
        nalgebra::core::Vector2::new(
            collidable.get_position().x,
            collidable.get_position().y),
        collidable.get_rotation());
    let aabb_cuboid2 = bounding_volume::aabb(&cuboid2, &cuboid2_pos);
    aabb_cuboid2
}

fn render_renderable(c: &Context, gl: &mut G2d, renderable: &Renderable) {
    let texture = &renderable.get_renderable_object().texture;
    let transform = c.transform
        .trans(renderable.get_position().x, renderable.get_position().y)
        .rot_rad(renderable.get_rotation())
        .trans((texture.get_size().0 as f64) * -0.5 * renderable.get_scale(),
               (texture.get_size().1 as f64) * -0.5 * renderable.get_scale())
        .scale(renderable.get_scale(), renderable.get_scale());
    image(texture.deref(), transform, gl);
}

use std::cell::RefCell;
use std::rc::Rc;

use ears::AudioController;
use piston_window::ImageSize;

use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::gun::{Gun, GUN_SCALE, PROJECTILE_VELOCITY_MAGNITUDE};
use crate::gun_strategy::GunStrategy;
use crate::gun_strategy_util::GunStrategyUtil;
use crate::object_type::ObjectType;
use crate::renderable_object::RenderableObject;
use crate::vector2::Vector2;
use crate::world::WorldReq;

pub struct ShotGun {
    pub should_delete: bool,
    pub gun_strategy_util: Rc<GunStrategyUtil>,
}

impl GunStrategy for ShotGun {
    fn get_should_delete(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::ShotGun
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        if other_object_type == ObjectType::Wall {
            self.set_should_delete(true);
        }
    }

    fn new_gun_strategy(&self) -> Box<GunStrategy> {
        Box::new(ShotGun {
            should_delete: false,
            gun_strategy_util: self.gun_strategy_util.clone(),
        })
    }

    fn has_gun_depth(&self) -> bool {
        false
    }

    fn get_gun_depth(&self) -> usize {
        0
    }

    fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq> {
        let new_guns = if self.guns.is_empty() {
            self.shoot_gun_from_player(player_pos, player_rot, mouse_pos)
        } else {
            let mut shot_guns: Vec<Rc<RefCell<Gun>>> = vec![]; // I see what you did there.
            let deepest_gun_depth = if let Some(last_gun) = self.guns.last() {
                last_gun.borrow().depth
            } else {
                0
            };
            for gun in self.guns.iter().rev() {
                if gun.borrow().depth != deepest_gun_depth {
                    break;
                }
                shot_guns.append(&mut self.make_guns());
            }
            shot_guns
        };

        self.guns.append(&mut new_guns.clone());
        self.shots_taken += 1;
        self.gun_strategy_util.world_requests_for_guns(new_guns)
    }
}

impl ShotGun {
    fn make_gun(&mut self, rotation: f64, velocity: Vector2, new_gun_rotation: f64) -> Gun {
        let vel = Vector2 {
            x: new_gun_rotation.cos(),
            y: new_gun_rotation.sin(),
        };
        let position = *self.get_position() + vel * 30.0;

        Gun {
            position,
            rotation,
            scale: GUN_SCALE,
            renderable_object: RenderableObject {
                texture: self.gun_texture.clone(),
            },
            selected_renderable_object: RenderableObject {
                texture: self.selected_gun_texture.clone(),
            },
            velocity,
            collidable_object: CollidableObject {
                width: f64::from(self.gun_texture.get_size().0),
                height: f64::from(self.gun_texture.get_size().1),
            },
            gun_sound: self.gun_sound.clone(),
            gun_texture: self.gun_texture.clone(),
            selected_gun_texture: self.selected_gun_texture.clone(),
            gun_strategy: self.gun_strategy.new_gun_strategy(),
            is_selected: true,
            depth: self.depth + 1,
        }
    }

    fn make_guns(&mut self) -> Vec<Rc<RefCell<Gun>>> {
        let rotation = self.get_rotation();

        let vel = Vector2 {
            x: rotation.cos(),
            y: rotation.sin(),
        };
        let velocity = vel * PROJECTILE_VELOCITY_MAGNITUDE;

        let gun1 = self.make_gun(rotation, velocity, rotation - 45.0);
        let gun2 = self.make_gun(rotation, velocity, rotation + 45.0);

        self.gun_sound.borrow_mut().play();

        vec![
            Rc::new(RefCell::new(gun1)),
            Rc::new(RefCell::new(gun2))
        ]
    }
}

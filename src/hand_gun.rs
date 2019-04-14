use std::cell::RefCell;
use std::rc::Rc;

use ears::AudioController;
use piston_window::ImageSize;

use crate::collidable_object::CollidableObject;
use crate::game_object::GameObject;
use crate::gun::Gun;
use crate::gun::GUN_SCALE;
use crate::gun::PROJECTILE_VELOCITY_MAGNITUDE;
use crate::gun_strategy::GunStrategy;
use crate::gun_strategy_util::GunStrategyUtil;
use crate::object_type::ObjectType;
use crate::renderable_object::RenderableObject;
use crate::vector2::Vector2;
use crate::world::WorldReq;

pub struct HandGun {
    pub should_delete: bool,
    pub gun_strategy_util: Rc<GunStrategyUtil>,
}

impl GunStrategy for HandGun {
    fn get_should_delete(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::HandGun
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        if other_object_type == ObjectType::Wall {
            self.set_should_delete(true);
        }
    }

    fn new_gun_strategy(&self) -> Box<GunStrategy> {
        Box::new(HandGun {
            should_delete: false,
            gun_strategy_util: self.gun_strategy_util.clone(),
        })
    }

    fn shoot_gun(&mut self, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<WorldReq> {
        let new_guns = if self.guns.is_empty() {
            self.gun_strategy_util.shoot_gun_from_player(player_pos, player_rot, mouse_pos)
        } else {
            let gun = self.guns.last().unwrap();
            self.is_selected = false;
            self.make_gun()
        };

        self.guns.append(&mut new_guns.clone());
        self.shots_taken += 1;
        self.gun_strategy_util.world_requests_for_guns(new_guns)
    }

    fn has_gun_depth(&self) -> bool {
        false
    }

    fn get_gun_depth(&self) -> usize {
        0
    }
}

impl HandGun{
    fn make_gun(&self) -> Vec<Rc<RefCell<Gun>>> {
        let rotation = self.get_rotation();

        let vel = Vector2 {
            x: rotation.cos(),
            y: rotation.sin(),
        };
        let velocity = vel * PROJECTILE_VELOCITY_MAGNITUDE;

        let position = *self.get_position() + (velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0;

        let gun = Gun {
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
        };

        self.gun_sound.borrow_mut().play();

        vec![Rc::new(RefCell::new(gun))]
    }
}

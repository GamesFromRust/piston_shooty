use crate::gun_behavior::GunBehavior;
use crate::object_type::ObjectType;
use crate::game_object::GameObject;
use piston_window::ImageSize;
use ears::AudioController;
use crate::gun::{Gun, PROJECTILE_VELOCITY_MAGNITUDE, GUN_SCALE};
use std::rc::Rc;
use std::cell::RefCell;
use crate::vector2::Vector2;
use crate::renderable_object::RenderableObject;
use crate::collidable_object::CollidableObject;

pub struct ShotGunBehavior {
    pub should_delete: bool,
}

impl GunBehavior for ShotGunBehavior {
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

    fn new_gun_behavior(&self) -> Box<GunBehavior> {
        Box::new(ShotGunBehavior {
            should_delete: false,
        })
    }

    fn has_gun_depth(&self) -> bool {
        false
    }

    fn get_gun_depth(&self) -> usize {
        0
    }

    fn shoot_gun(&self, gun: &Gun) -> Vec<Rc<RefCell<Gun>>> {
        let rotation = gun.get_rotation();

        let vel = Vector2 {
            x: rotation.cos(),
            y: rotation.sin(),
        };
        let velocity = vel * PROJECTILE_VELOCITY_MAGNITUDE;

        let gun1 = self.make_gun(gun, rotation, velocity, rotation - 45.0);
        let gun2 = self.make_gun(gun, rotation, velocity, rotation + 45.0);

        gun.gun_sound.borrow_mut().play();

        vec![
            Rc::new(RefCell::new(gun1)),
            Rc::new(RefCell::new(gun2))
        ]
    }
}

impl ShotGunBehavior {
    fn make_gun(&self, gun: &Gun, rotation: f64, velocity: Vector2, new_gun_rotation: f64) -> Gun {
        let vel = Vector2 {
            x: new_gun_rotation.cos(),
            y: new_gun_rotation.sin(),
        };
        let position = *gun.get_position() + vel * 30.0;

        Gun {
            position,
            rotation,
            scale: GUN_SCALE,
            renderable_object: RenderableObject {
                texture: gun.gun_texture.clone(),
            },
            selected_renderable_object: RenderableObject {
                texture: gun.selected_gun_texture.clone(),
            },
            velocity,
            collidable_object: CollidableObject {
                width: f64::from(gun.gun_texture.get_size().0),
                height: f64::from(gun.gun_texture.get_size().1),
            },
            gun_sound: gun.gun_sound.clone(),
            gun_texture: gun.gun_texture.clone(),
            selected_gun_texture: gun.selected_gun_texture.clone(),
            gun_behavior: gun.gun_behavior.new_gun_behavior(),
            is_selected: true,
            depth: gun.depth + 1,
        }
    }
}

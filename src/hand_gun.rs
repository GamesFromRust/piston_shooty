use crate::gun_strategy::GunStrategy;
use crate::object_type::ObjectType;
use std::rc::Rc;
use std::cell::RefCell;
use crate::gun::Gun;
use crate::vector2::Vector2;
use crate::renderable_object::RenderableObject;
use crate::collidable_object::CollidableObject;
use crate::gun::GUN_SCALE;
use crate::gun::PROJECTILE_VELOCITY_MAGNITUDE;
use crate::game_object::GameObject;
use piston_window::ImageSize;
use ears::AudioController;

pub struct HandGun {
    pub should_delete: bool,
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

        let position = *gun.get_position() + (velocity / PROJECTILE_VELOCITY_MAGNITUDE) * 30.0;

        let gun = Gun {
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
            gun_strategy: gun.gun_strategy.new_gun_strategy(),
            is_selected: true,
            depth: gun.depth + 1,
        };

        gun.gun_sound.borrow_mut().play();

        vec![Rc::new(RefCell::new(gun))]
    }
}

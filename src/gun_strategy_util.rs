use std::cell::RefCell;
use std::rc::Rc;

use crate::collidable_object::CollidableObject;
use crate::gun::Gun;
use crate::gun::GUN_SCALE;
use crate::gun::PROJECTILE_VELOCITY_MAGNITUDE;
use crate::gun_strategy::GunStrategy;
use crate::renderable_object::RenderableObject;
use crate::vector2::Vector2;
use crate::world::WorldReq;
use crate::world::WorldRequestType;

pub struct GunStrategyUtil {}

impl GunStrategyUtil {
    pub fn shoot_gun_from_player(self, &mut gun_strategy: &GunStrategy, player_pos: &Vector2, player_rot: f64, mouse_pos: &Vector2) -> Vec<Rc<RefCell<Gun>>> {
        let velocity = (*mouse_pos - *player_pos).normalized() * PROJECTILE_VELOCITY_MAGNITUDE;

        let gun = Gun {
            position: *player_pos,
            rotation: player_rot,
            scale: GUN_SCALE,
            renderable_object: RenderableObject {
                texture: gun_strategy.gun_texture.clone(),
            },
            selected_renderable_object: RenderableObject {
                texture: gun_strategy.selected_gun_texture.clone(),
            },
            velocity,
            collidable_object: CollidableObject {
                width: f64::from(gun_strategy.gun_texture.get_size().0),
                height: f64::from(gun_strategy.gun_texture.get_size().1),
            },
            gun_sound: gun_strategy.gun_sound.clone(),
            gun_texture: gun_strategy.gun_texture.clone(),
            selected_gun_texture: gun_strategy.selected_gun_texture.clone(),
            gun_strategy: gun_strategy.new_gun_strategy(),
            is_selected: true,
            depth: 0,
        };

        gun_strategy.gun_sound.borrow_mut().play();

        vec![Rc::new(RefCell::new(gun))]
    }

    // TODO: DUPLICATES world_requests_for_bullet
    pub fn world_requests_for_guns(&self, guns: Vec<Rc<RefCell<Gun>>>) -> Vec<WorldReq> {
        // TODO: https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting

        let mut world_reqs: Vec<WorldReq> = vec![];

        for gun in guns {
            self.world_requests_for_gun(gun, &mut world_reqs);
        }

        world_reqs
    }

    fn world_requests_for_gun(&self, gun: Rc<RefCell<Gun>>, world_reqs: &mut Vec<WorldReq>) {
        let world_req: WorldReq = WorldReq {
            renderable: Some(gun.clone()),
            updatable: None,
            collidable: Some(gun.clone()),
            req_type: WorldRequestType::AddDynamicRenderable,
        };
        world_reqs.push(world_req);
        let world_req: WorldReq = WorldReq {
            renderable: None,
            updatable: Some(gun.clone()),
            collidable: None,
            req_type: WorldRequestType::AddUpdatable,
        };
        world_reqs.push(world_req);
    }
}
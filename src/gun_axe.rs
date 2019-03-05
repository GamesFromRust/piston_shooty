use crate::gun_strategy::GunStrategy;
use crate::object_type::ObjectType;

pub struct GunAxe {
    pub should_delete: bool,
}

impl GunStrategy for GunAxe {
    fn get_should_delete(&self) -> bool {
        self.should_delete
    }

    fn set_should_delete(&mut self, should_delete: bool) {
        self.should_delete = should_delete
    }

    fn get_object_type(&self) -> ObjectType {
        ObjectType::GunAxe
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        if other_object_type == ObjectType::Wall {
            self.set_should_delete(true);
        }
    }

    fn new_gun_strategy(&self) -> Box<GunStrategy> {
        Box::new(GunAxe {
            should_delete: false,
        })
    }

    fn has_gun_depth(&self) -> bool {
        true
    }

    fn get_gun_depth(&self) -> usize {
        2
    }
}

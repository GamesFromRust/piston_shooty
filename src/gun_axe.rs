use object_type::ObjectType;
use gun_strategy::GunStrategy;
use gun::Gun;
use game_object::GameObject;

pub struct GunAxe {
    pub should_delete: bool
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
        match other_object_type {
            ObjectType::Wall => {
                self.set_should_delete(true);
            },
            _ => {},
        }
    }
}
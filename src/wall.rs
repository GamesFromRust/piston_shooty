use renderable_object::RenderableObject;
use renderable::Renderable;
use object_type::ObjectType;
use collidable_object::CollidableObject;
use collidable::Collidable;
use vector2::Vector2;
use game_object::GameObject;

pub struct Wall {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: f64,
    pub renderable_object: RenderableObject,
    pub collidable_object: CollidableObject,
}

impl GameObject for Wall {
    fn get_position(&self) -> &Vector2 {
        &self.position
    }

    fn get_rotation(&self) -> f64 {
        self.rotation
    }
    
    fn get_scale(&self) -> f64 {
        self.scale
    }
    
    fn get_should_delete(&self) -> bool {
        false
    }
    
    #[allow(unused_variables)]
    fn set_should_delete(&mut self, should_delete: bool) {
        // do nothing
    }
    
    fn get_object_type(&self) -> ObjectType {
        ObjectType::Wall
    }
}

impl Renderable for Wall {
    fn get_renderable_object(&self) -> &RenderableObject {
        &self.renderable_object
    }
}

impl Collidable for Wall {
    fn get_collidable_object(&self) -> &CollidableObject {
        &self.collidable_object
    }

    fn collide(&mut self, other_object_type: ObjectType) {
        
    }
}

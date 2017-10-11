use object_type::ObjectType;

pub trait Collidable {
    fn collide(&self, other_object_type: ObjectType);
}
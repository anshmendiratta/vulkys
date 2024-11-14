use super::rigidbody::{GenericObject, Updateable};

struct Collision {
    collision_type: CollisionObjectType,
    other: dyn GenericObject,
}

pub enum CollisionObjectType {
    World,
    Object,
}

pub trait CollisionHandler<Obj: Updateable> {
    fn check_collision(&self) -> Option<CollisionObjectType> {
        None
    }
    fn resolve_world_collision(&mut self) {}
    fn resolve_object_collision(&mut self) {}
}

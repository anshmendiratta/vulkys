use super::rigidbody::{GenericObject, Updateable};

struct Collision {
    collision_type: CollisionObjectType,
    other: dyn GenericObject,
}

enum CollisionObjectType {
    World,
    Object,
}

pub trait CollisionHandler<Obj: Updateable> {
    pub fn check_collision(&self) -> Option<CollisionObjectType> {}
    pub fn resolve_world_collision(&mut self) {}
    pub fn resolve_object_collision(&mut self) {}
}

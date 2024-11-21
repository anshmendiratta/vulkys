use super::rigidbody::GenericObject;

struct Collision {
    collision_type: CollisionObjectType,
    other: dyn GenericObject,
}

pub enum CollisionObjectType {
    World,
    Object,
}

pub trait CollisionHandler {
    fn check_collision(&self) -> Option<CollisionObjectType> {
        None
    }
    fn resolve_world_collision(&mut self) {}
    fn resolve_object_collision(&mut self) {}
}

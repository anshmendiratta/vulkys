use super::rigidbody::RigidBody;

pub struct Collision {
    collision_type: CollisionObjectType,
    other: Option<RigidBody>,
}

impl Collision {
    pub fn new(collision_type: CollisionObjectType, other_object: Option<RigidBody>) -> Self {
        Self {
            collision_type,
            other: other_object,
        }
    }
    pub fn get_collision_type(&self) -> CollisionObjectType {
        self.collision_type.clone()
    }
    pub fn get_other_object(&self) -> Option<RigidBody> {
        self.other.clone()
    }
}

#[derive(Clone)]
pub enum CollisionObjectType {
    World,
    Object,
}

pub trait CollisionHandler {
    fn check_collisions(&self) -> (Option<Vec<Collision>>, (bool, bool));
    fn resolve_object_collision(&mut self);
    fn resolve_world_collision(&mut self, crossed_boundaries_xy: (bool, bool));
}

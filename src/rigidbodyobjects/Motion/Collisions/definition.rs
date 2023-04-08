use crate::rigidbodyobjects::rigidbodies::RigidBody;
use crate::world::World;

enum Collision {
    ObjObj(RigidBody, RigidBody),
    ObjWorld(RigidBody, World)
}

#[derive(Debug)]
pub struct CollisionData {
    objects: Collision,
    time: f32,
}

impl CollisionData {
    pub fn get_time(&self) -> f32 {
        &self.time
    }

    pub fn get_object(&self) -> RigidBody {
        &self.object
    }
}

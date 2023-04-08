use crate::rigidbodyobjects::rigidbodies::RigidBody;
use crate::world::World;

enum Collision {
    ObjObj(RigidBody, RigidBody),
    ObjWorld(RigidBody, World)
}

#[derive(Debug)]
pub struct CollisionData {
    object: Collision,
    time: f32,
}

impl CollisionData {
    pub fn get_time(&self) -> &f32 {
        &self.time
    }

    pub fn get_objects<T>(&self) -> Vec<T> {
        if let Collision::ObjObj(body1, body2) = self.object {
            vec![body1, body2]
        } else if let Collision::ObjWorld(body, world) = self.object {
            vec![body, world]
        } 
    }
}

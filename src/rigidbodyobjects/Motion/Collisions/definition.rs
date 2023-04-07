use crate::rigidbodyobjects::rigidbodies::RigidBody;
use crate::world::World;

#[derive(Debug)]
pub struct Collision {
    objects: Vec<2, RigidBody, BodyOrWorld>, 
    time: f32,
}

enum BodyOrWorld {
    RigidBody,
    World
}

impl Collision {
    pub fn get_time(&self) -> f32 {
        &self.time
    }

    pub fn get_axis(&self) -> &str {
        &self.axis
    }

    pub fn get_object(&self) -> RigidBody {
        &self.object
    }
}

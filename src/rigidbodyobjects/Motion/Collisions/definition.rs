use crate::rigidbodyobjects::rigidbodies::RigidBody;
use crate::world::World;

#[derive(Debug)]
pub struct Collision {
    objects: (RigidBody, Either<RigidBody, World>), 
    time: f32,
}

enum Either<A, B> {
    Left(A),
    Right(B)
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

use crate::resolve_collisions;
use crate::rigidbodies::RigidBody;
use crate::world::*;

#[derive(Debug)]
pub enum Collision {
    ObjObj(RigidBody, RigidBody),
    ObjWorld(RigidBody, World),
}

#[derive(Debug)]
pub struct CollisionData {
    objects: Collision,
    time: f64,
}

impl Collision {
    fn get_participants(&self) -> (&RigidBody, Option<&RigidBody>) {
        match &self {
            Collision::ObjObj(body1, body2) => (body1, Some(body2)),
            Collision::ObjWorld(body1, world) => (body1, None),
        }
    }
}

impl CollisionData {
    pub fn get_time(&self) -> &f64 {
        &self.time
    }

    pub fn get_objects(&self) -> (&RigidBody, Option<&RigidBody>) {
        *&self.objects.get_participants()
    }
}

// pub fn match_collision_objects*&self, collision: &Collision) {
//     match collision {

//     }

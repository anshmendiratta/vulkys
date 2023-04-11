use crate::rigidbodyobjects::rigidbodies::RigidBody;
use crate::world::World;

#[derive(Debug)]
enum Collision {
    ObjObj(RigidBody, RigidBody),
    ObjWorld(RigidBody, World)
}

#[derive(Debug)]
pub struct CollisionData {
    object: Collision,
    time: f32,
}

enum RigidWorldVec {
    
}

impl CollisionData {
    pub fn get_time(&self) -> &f32 {
        &self.time
    }

    pub fn get_objects(&self) -> &Collision {
        match &self.object {
            Collision::ObjWorld(body, world) => handle_world_body_collision(body, world),
            Collision::ObjObj(body1, body2) => handle_body_body_collision(body1, body2)
        }
    } 

    // pub fn match_collision_objects(&self, collision: &Collision) {
    //     match collision {
            
    //     }
}

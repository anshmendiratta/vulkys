use crate::FVec2;

use super::rigidbody::RigidBody;

pub struct Collision {
    collision_type: CollisionObjectType,
    primary: Option<RigidBody>,
    secondary: Option<RigidBody>,
}

impl Collision {
    pub fn new(
        collision_type: CollisionObjectType,
        primary_object: Option<RigidBody>,
        other_object: Option<RigidBody>,
    ) -> Self {
        Self {
            collision_type,
            primary: primary_object,
            secondary: other_object,
        }
    }
    pub fn get_collision_type(&self) -> CollisionObjectType {
        self.collision_type.clone()
    }
    pub fn get_primary(&self) -> Option<RigidBody> {
        self.primary.clone()
    }
    pub fn get_secondary(&self) -> Option<RigidBody> {
        self.secondary.clone()
    }
    pub fn resolve_objects_and_return_secondary_velocity(&mut self) -> FVec2 {
        // NOTE: Only solving the circle case
        // NOTE: Both objects should also exist. Otherwise invalid construction
        let first = self.primary.as_mut().unwrap();
        let second = self.secondary.as_mut().unwrap();
        let vector_between_com = first.get_position() - second.get_position();
        let unit_axis_of_seperation = vector_between_com.get_orthogonal_unit();
        let updated_secondary_velocity =
            second.get_velocity().mirror_along(unit_axis_of_seperation);

        updated_secondary_velocity
    }
}

#[derive(Clone)]
pub enum CollisionObjectType {
    World,
    Object,
}

pub trait CollisionHandler {
    fn check_collisions(&self) -> (Option<Vec<Collision>>, (bool, bool));
    // fn resolve_object_collision(&mut self);
    fn resolve_world_collision(&mut self, crossed_boundaries_xy: (bool, bool));
}

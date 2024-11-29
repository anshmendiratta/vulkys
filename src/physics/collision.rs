use libm::{cos, sin};
use tracing::info;

use crate::FVec2;

use super::rigidbody::RigidBody;

#[derive(Debug)]
pub struct Collision {
    collision_type: CollisionObjectType,
    primary: Option<RigidBody>,
    secondary: Option<RigidBody>,
}

#[derive(Clone, Debug)]
pub enum CollisionObjectType {
    World,
    Object,
}

type ObjectPosVelUpdates = [[FVec2; 2]; 2];
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
    #[doc = "Returns the updates as [[position, velocity]] for each object in the collision, marking the reference as 'primary'"]
    pub fn resolve_objects_and_return_updates(&self) -> ObjectPosVelUpdates {
        // NOTE: Only solving the circle case
        // NOTE: Both objects should also exist. Otherwise invalid construction
        let primary = self.primary.clone().unwrap();
        let secondary = self.secondary.clone().unwrap();
        let vector_between_com = primary.get_position() - secondary.get_position();
        let unit_axis_of_seperation = vector_between_com.get_orthogonal_unit();
        // NOTE: Uses https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
        let updated_primary_velocity = primary.get_velocity()
            - (primary.get_position() - secondary.get_position()).scale(
                (primary.get_velocity() - secondary.get_velocity())
                    .dot(primary.get_position() - secondary.get_position())
                    / vector_between_com.magnitude().powf(2.),
            );
        let updated_secondary_velocity = secondary.get_velocity()
            - (secondary.get_position() - primary.get_position()).scale(
                (secondary.get_velocity() - primary.get_velocity())
                    .dot(secondary.get_position() - primary.get_position())
                    / vector_between_com.magnitude().powf(2.),
            );

        // Check that the conservation of momentum still holds
        assert!(
            (primary.get_velocity() + secondary.get_velocity()
                - updated_primary_velocity
                - updated_secondary_velocity)
                .magnitude()
                < 0.01
        );

        let distance_to_move_coms =
            primary.get_radius() + secondary.get_radius() - vector_between_com.magnitude();
        let primary_velocity_polar_angle = primary.get_velocity().get_polar_angle();
        let secondary_velocity_polar_angle = primary.get_velocity().get_polar_angle();
        let distance_to_move_primary = FVec2::new(
            distance_to_move_coms * cos(primary_velocity_polar_angle as f64) as f32,
            distance_to_move_coms * sin(primary_velocity_polar_angle as f64) as f32,
        ) * 2.;
        let distance_to_move_secondary = FVec2::new(
            distance_to_move_coms * cos(secondary_velocity_polar_angle as f64) as f32,
            distance_to_move_coms * sin(secondary_velocity_polar_angle as f64) as f32,
        ) * 2.;
        let updated_primary_position = primary.get_position() + distance_to_move_primary;
        let updated_secondary_position = secondary.get_position() + distance_to_move_secondary;

        info!(
            "changed position and velocity of id={} circle with r={} from p={}->{}, v={}->{}",
            primary.get_id(),
            primary.get_radius(),
            primary.get_position(),
            updated_primary_position,
            primary.get_velocity(),
            updated_primary_velocity
        );
        info!(
            "changed position and velocity of id={} circle with r={} from p={}->{}, v={}->{}",
            secondary.get_id(),
            secondary.get_radius(),
            secondary.get_position(),
            updated_secondary_position,
            secondary.get_velocity(),
            updated_secondary_velocity
        );

        [
            [updated_primary_position, updated_primary_velocity],
            [updated_secondary_position, updated_secondary_velocity],
        ]
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        if let (None, None, None, None) = (
            self.get_primary(),
            self.get_secondary(),
            other.get_primary(),
            other.get_secondary(),
        ) {
            return true;
        }

        let self_primary = self.get_primary().unwrap();
        let self_secondary = self.get_secondary().unwrap();
        let other_primary = other.get_primary().unwrap();
        let other_secondary = other.get_secondary().unwrap();
        match (
            self_primary == other_primary,
            self_primary == other_secondary,
            self_secondary == other_primary,
            self_secondary == other_secondary,
        ) {
            (true, false, true, false) | (false, true, false, true) => return true,
            /* the case with `Some` objects and all trues should never occur  */
            _ => return false,
        };
    }
}

pub trait CollisionHandler {
    fn check_world_collisions(&self) -> (Option<Vec<Collision>>, (bool, bool));
    fn resolve_world_collision(&mut self, crossed_boundaries_xy: (bool, bool));
}

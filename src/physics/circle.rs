use ecolor::Color32;

use crate::FVec2;

use super::{
    collision::{Collision, CollisionHandler, CollisionObjectType, WorldCollisionInfo},
    lib::{COEFF_RESTITUTION, WORLD_BOUNDS},
    rigidbody::GenericObject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub position: FVec2,
    pub velocity: FVec2,
    pub color: Color32,
}

impl GenericObject for Circle {
    fn get_debug(&self) -> String {
        format!(
            "r = {},
                p = {},
                v = {}",
            self.radius, self.position, self.velocity
        )
    }
    fn get_radius(&self) -> f32 {
        self.radius
    }
    fn get_position(&self) -> FVec2 {
        self.position
    }
    fn get_color(&self) -> Color32 {
        self.color
    }
}

impl CollisionHandler for Circle {
    fn check_world_collisions(&self) -> Option<Collision> {
        let y_pos_range = self.position.y - self.radius..self.position.y + self.radius;
        let x_pos_range = self.position.x - self.radius..self.position.x + self.radius;
        let in_x_bounds = WORLD_BOUNDS.0.contains(&x_pos_range.start)
            && WORLD_BOUNDS.0.contains(&x_pos_range.end);
        let in_y_bounds = WORLD_BOUNDS.1.contains(&y_pos_range.start)
            && WORLD_BOUNDS.1.contains(&y_pos_range.end);

        if !(in_y_bounds && in_x_bounds) {
            let world_collision = WorldCollisionInfo::new(in_x_bounds, in_y_bounds);
            let collision = Collision::new(CollisionObjectType::World(world_collision), None, None);
            return Some(collision);
        };

        None
    }
    fn resolve_world_collision(&mut self, in_boundaries_xy: WorldCollisionInfo) {
        let mut distance_to_offset = FVec2::new(0., 0.);
        let position = self.get_position();
        if !in_boundaries_xy.get_crossed_x() {
            self.velocity.x *= -1. * COEFF_RESTITUTION;
            if position.x + self.get_radius() > WORLD_BOUNDS.0.end {
                distance_to_offset.x = WORLD_BOUNDS.0.end - position.x - self.get_radius();
            } else if position.x + self.get_radius() < WORLD_BOUNDS.0.start {
                distance_to_offset.x = -WORLD_BOUNDS.0.start - position.x + self.get_radius();
            }
        }
        if !in_boundaries_xy.get_crossed_y() {
            self.velocity.y *= -1. * COEFF_RESTITUTION;
            if position.y + self.get_radius() > WORLD_BOUNDS.1.end {
                distance_to_offset.y = WORLD_BOUNDS.1.end - position.y - self.get_radius();
            } else if position.x + self.get_radius() > WORLD_BOUNDS.1.end {
                distance_to_offset.y = -WORLD_BOUNDS.1.start - position.y + self.get_radius();
            }
        }

        self.position += distance_to_offset * 2.;
    }
}

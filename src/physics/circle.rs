use ecolor::Color32;
use serde::Serialize;

use crate::FVec2;

use super::{
    collision::{Collision, CollisionHandler, CollisionObjectType},
    lib::{COEFF_RESTITUTION, WORLD_BOUNDS},
    rigidbody::GenericObject,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
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
    fn check_collisions(&self) -> (Option<Vec<Collision>>, (bool, bool)) {
        let y_pos_range = self.position.y - self.radius..self.position.y + self.radius;
        let x_pos_range = self.position.x - self.radius..self.position.x + self.radius;
        let in_x_bounds = WORLD_BOUNDS.0.contains(&x_pos_range.start)
            && WORLD_BOUNDS.0.contains(&x_pos_range.end);
        let in_y_bounds = WORLD_BOUNDS.1.contains(&y_pos_range.start)
            && WORLD_BOUNDS.1.contains(&y_pos_range.end);

        if !(in_y_bounds && in_x_bounds) {
            let collision = Collision::new(CollisionObjectType::World, None);
            return (Some(vec![collision]), (in_x_bounds, in_y_bounds));
        };

        (None, (false, false))
    }

    fn resolve_world_collision(&mut self, crossed_boundaries_xy: (bool, bool)) {
        if !crossed_boundaries_xy.0 {
            self.velocity.x *= -1. * COEFF_RESTITUTION;
        }
        if !crossed_boundaries_xy.1 {
            self.velocity.y *= -1. * COEFF_RESTITUTION;
        }
    }
    fn resolve_object_collision(&mut self) {
        todo!()
    }
}

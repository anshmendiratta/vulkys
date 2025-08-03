use ecolor::Color32;
use glm::Vec3;
use nalgebra::Rotation3;

use crate::{
    render::cube::{CUBE_INDICES, CUBE_VERTICES},
    vulkan::core::CustomVertex,
};

use super::rigidbody::GenericObject;

#[derive(Debug, Clone, PartialEq)]
pub struct RawCuboid {
    pub half_extent: Vec3, // Doubles as the "scalars" for the cuboid.
    pub init_position: Vec3,
    pub init_velocity: Vec3,
    pub rotation: Rotation3<f32>,
    pub color: Color32,
}

impl GenericObject for RawCuboid {
    fn get_debug(&self) -> String {
        format!(
            "half_extent = {},
                p = {},
                v = {}",
            self.half_extent, self.init_position, self.init_velocity
        )
    }

    fn get_color(&self) -> Color32 {
        self.color
    }

    fn get_radius(&self) -> f32 {
        (self.half_extent.x.powf(2.) + self.half_extent.y.powf(2.)).powf(0.5)
    }

    fn get_init_position(&self) -> Vec3 {
        self.init_position
    }

    fn get_init_velocity(&self) -> Vec3 {
        self.init_velocity
    }

    fn get_vertices(&self) -> Vec<CustomVertex> {
        vec![]
        // self.get_vertices()
    }

    fn get_indices(&self) -> Vec<u16> {
        self.get_vertex_indices()
    }
}

impl RawCuboid {
    pub fn get_half_extent(&self) -> Vec3 {
        self.half_extent
    }

    pub fn get_orientation(&self) -> Rotation3<f32> {
        self.rotation
    }

    pub fn get_vertices(&self) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = CUBE_VERTICES.clone().to_vec();
        let scalars = self.half_extent.map(|e| e * 2.);

        // Scales.
        vertices = vertices
            .iter_mut()
            .map(|v| {
                v.x *= scalars.x;
                v.y *= scalars.y;
                v.z *= scalars.z;
                *v
            })
            .collect::<Vec<_>>();

        dbg!(&self.rotation);
        // Rotation.
        vertices = vertices
            .iter_mut()
            .map(|v| self.rotation * *v)
            .collect::<Vec<_>>();

        dbg!(&vertices);

        vertices
    }

    pub fn get_vertex_indices(&self) -> Vec<u16> {
        CUBE_INDICES.to_vec()
    }
}

// impl CollisionHandler for Square  {
//     fn check_world_collisions(&self) -> Option<Collision> {
//         let y_pos_range = self.position.y - self.radius..self.position.y + self.radius;
//         let x_pos_range = self.position.x - self.radius..self.position.x + self.radius;
//         let in_x_bounds = WORLD_BOUNDS.0.contains(&x_pos_range.start)
//             && WORLD_BOUNDS.0.contains(&x_pos_range.end);
//         let in_y_bounds = WORLD_BOUNDS.1.contains(&y_pos_range.start)
//             && WORLD_BOUNDS.1.contains(&y_pos_range.end);

//         if !(in_y_bounds && in_x_bounds) {
//             let world_collision = WorldCollisionInfo::new(in_x_bounds, in_y_bounds);
//             let collision = Collision::new(CollisionObjectType::World(world_collision), None, None);
//             return Some(collision);
//         };

//         None
//     }
//     fn resolve_world_collision(&mut self, in_boundaries_xy: WorldCollisionInfo) {
//         let mut distance_to_offset = Vec3::new(0., 0.);
//         let position = self.get_position();
//         if !in_boundaries_xy.get_crossed_x() {
//             self.velocity.x *= -1. * COEFF_RESTITUTION;
//             if position.x + self.get_radius() > WORLD_BOUNDS.0.end {
//                 distance_to_offset.x = WORLD_BOUNDS.0.end - position.x - self.get_radius();
//             } else if position.x + self.get_radius() < WORLD_BOUNDS.0.start {
//                 distance_to_offset.x = -WORLD_BOUNDS.0.start - position.x + self.get_radius();
//             }
//         }
//         if !in_boundaries_xy.get_crossed_y() {
//             self.velocity.y *= -1. * COEFF_RESTITUTION;
//             if position.y + self.get_radius() > WORLD_BOUNDS.1.end {
//                 distance_to_offset.y = WORLD_BOUNDS.1.end - position.y - self.get_radius();
//             } else if position.x + self.get_radius() > WORLD_BOUNDS.1.end {
//                 distance_to_offset.y = -WORLD_BOUNDS.1.start - position.y + self.get_radius();
//             }
//         }

//         self.position += distance_to_offset * 2.;
//     }
// }

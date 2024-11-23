use std::collections::HashMap;
use std::hash::RandomState;

use crate::{
    renderer::{vk_core::WindowEventHandler, vk_proc_func::Polygon},
    FVec2,
};

use super::{
    lib::{DELTA_TIME, GRAVITY_ACCELERATION},
    rigidbody::RigidBody,
};

pub struct Scene {
    pub objects: Vec<RigidBody>,
    pub objects_hash: HashMap<u8, (RigidBody, Polygon)>,
    dt: f32,
}

impl Scene {
    pub fn with_objects(mut objects: Vec<RigidBody>) -> Self {
        objects.iter_mut().for_each(|obj| {
            let mut current_position = obj.get_position();
            current_position.y *= -1.;
            let updated_position = FVec2::new(current_position.x, current_position.y);
            obj.update_position(updated_position);
        });

        let polygons: Vec<Polygon> = objects.iter().map(|body| body.to_polygon()).collect();

        let mut objects_hash: HashMap<u8, (RigidBody, Polygon)> =
            HashMap::with_capacity_and_hasher(objects.len(), RandomState::new());
        for (rigidbody, polygon) in std::iter::zip(&objects, polygons) {
            objects_hash.insert(rigidbody.get_id(), (rigidbody.clone(), polygon));
        }

        Self {
            objects,
            dt: DELTA_TIME,
            objects_hash,
        }
    }
    // TODO: modify the `run_with_objects` to loop over itself with updated values instead of being called in a loop
    // Currently runs the window context handler function over and over again.
    // That is BAD.
    pub fn run(self) {
        let windowcx_handler = WindowEventHandler::new();
        windowcx_handler.run_with_scene(self);
    }
    pub fn update_objects(&mut self) {
        for object in &mut self.objects {
            let current_velocity = object.get_velocity();
            let updated_velocity = FVec2::new(
                current_velocity.x,
                current_velocity.y + GRAVITY_ACCELERATION * self.dt,
            );
            let current_position = object.get_position();
            let updated_position = FVec2::new(
                current_position.x + current_velocity.x * self.dt,
                current_position.y + current_velocity.y * self.dt,
            );
            object.update_velocity(updated_velocity);
            object.update_position(updated_position);
        }
    }
    pub fn recreate_hash(&mut self) {
        let polygons: Vec<Polygon> = self.objects.iter().map(|body| body.to_polygon()).collect();

        let mut objects_as_hash: HashMap<u8, (RigidBody, Polygon)> =
            HashMap::with_capacity_and_hasher(self.objects.len(), RandomState::new());
        for (rigidbody, polygon) in std::iter::zip(&self.objects, polygons) {
            objects_as_hash.insert(rigidbody.get_id(), (rigidbody.clone(), polygon));
        }

        self.objects_hash = objects_as_hash;
    }
}

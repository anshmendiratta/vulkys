use std::collections::HashMap;
use std::hash::RandomState;

use crate::{
    renderer::{vk_core::WindowEventHandler, vk_proc_func::Polygon},
    FVec2,
};

use super::{
    collision::{Collision, CollisionHandler, CollisionObjectType},
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
        // NOTE: correcting for the vulkan coordinate system; resetting to (0,0) being lower left
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
    pub fn run(self) {
        let windowcx_handler = WindowEventHandler::new();
        windowcx_handler.run_with_scene(self);
    }
    pub fn check_and_resolve_collision(&mut self) {
        // Checking for object-world collisions
        for object in &mut self.objects {
            let world_collisions: (Option<Vec<Collision>>, (bool, bool)) =
                object.check_collisions();
            if let (Some(collision_types), (x, y)) = world_collisions {
                match collision_types[0].get_collision_type() {
                    CollisionObjectType::World => object.resolve_world_collision((x, y)),
                    _ => (),
                }
            }
        }

        let mut did_resolve_object_collisions: bool = false;
        // Checking for object-object collisions
        for ref_object in &self.objects {
            let mut object_collisions: Vec<Collision> = Vec::new();
            self.objects.iter().for_each(|checking_object| {
                if checking_object == ref_object
                    || (ref_object.get_position() - checking_object.get_position()).magnitude()
                        >= (ref_object.get_radius() + checking_object.get_radius())
                {
                    return;
                }
                object_collisions.push(Collision::new(
                    CollisionObjectType::Object,
                    Some(ref_object.clone()),
                    Some(checking_object.clone()),
                ));
            });

            object_collisions.iter_mut().for_each(|collision| {
                let (ref_id, checking_id) = (
                    collision.get_primary().unwrap().get_id(),
                    collision.get_secondary().unwrap().get_id(),
                );
                let updated_velocity = collision.resolve_objects_and_return_secondary_velocity();
                self.objects_hash
                    .entry(checking_id)
                    .and_modify(|(affected_object, _)| {
                        affected_object.update_velocity(updated_velocity);
                    });
            });

            if !object_collisions.is_empty() {
                did_resolve_object_collisions = true;
            }
        }
        if did_resolve_object_collisions {
            self.recreate_objects_from_hash();
        }
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
        self.check_and_resolve_collision();
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
    pub fn recreate_objects_from_hash(&mut self) {
        self.objects.clear();
        for (rigidbody, _) in self.objects_hash.values() {
            self.objects.push(rigidbody.clone());
        }
    }
}

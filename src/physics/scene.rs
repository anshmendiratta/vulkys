use std::hash::RandomState;
use std::{collections::HashMap, sync::Arc};

use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::sync::{self, GpuFuture};
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};
use winit::event_loop::EventLoop;

use crate::renderer::vk_core::VulkanoContext;
use crate::renderer::vk_primitives::get_compute_command_buffer;
use crate::{
    renderer::{
        vk_core::{CustomVertex, WindowEventHandler},
        vk_primitives::create_memory_allocator,
        vk_procedural_functions::{Polygon, PolygonMethods},
    },
    FVec2,
};

use super::{
    collision::{Collision, CollisionHandler, CollisionObjectType},
    lib::{DELTA_TIME, GRAVITY_ACCELERATION},
    rigidbody::RigidBody,
};

pub mod update_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            layout(push_constant) uniform ComputeConstants {
                float gravity;
                float dt;
            };
            
            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 0, set = 0) buffer P {
                vec2 pos[];
            } positions;
            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 1, set = 0) buffer V {
                vec2 vel[];
            } velocities;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                vec2 position_change = vec2(velocities.vel[idx] * dt);
                vec2 velocity_change = vec2(0, gravity * dt);

                positions.pos[idx] += position_change;
                velocities.vel[idx] += velocity_change;
            }
        ",
    }
}

pub struct Scene {
    pub objects: Vec<RigidBody>,
    pub objects_hash: HashMap<u8, (RigidBody, Polygon)>,
    dt: f32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
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

    pub fn return_objects_as_vertex_buffer(
        &self,
        device: Arc<Device>,
    ) -> Subbuffer<[CustomVertex]> {
        let vertex_buffer_data = {
            let mut buffer_data: Vec<CustomVertex> =
                Vec::with_capacity(self.objects_hash.len() * 3);
            for (_, (_, polygon)) in &self.objects_hash {
                buffer_data = [buffer_data, polygon.destructure_into_list()].concat();
            }
            buffer_data
        };
        Buffer::from_iter(
            create_memory_allocator(device.clone()),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertex_buffer_data.clone(),
        )
        .expect("scene: could not produce vertex buffer from objects")
    }

    pub fn run(self) {
        let event_loop = EventLoop::new();
        let windowcx_handler = WindowEventHandler::new(&event_loop);
        windowcx_handler.run_with_scene(self, event_loop);
    }

    pub fn check_and_resolve_collision(&mut self) {
        // Checking for object-world collisions
        for object in &mut self.objects {
            let world_collisions: (Option<Vec<Collision>>, (bool, bool)) =
                object.check_world_collisions();
            if world_collisions.0.is_none() {
                continue;
            }
            world_collisions.0.unwrap().iter().for_each(|collision| {
                match collision.get_collision_type() {
                    CollisionObjectType::World => object
                        .resolve_world_collision((world_collisions.1 .0, world_collisions.1 .1)),
                    _ => (),
                }
            })
        }

        let mut did_resolve_object_collisions: bool = false;
        // Checking for object-object collisions
        let mut object_collisions: Vec<Collision> = Vec::new();
        for ref_object in &self.objects {
            self.objects.iter().for_each(|checking_object| {
                if /* skip checking itself */ checking_object == ref_object
                    || /* no detected collision */ (ref_object.get_position() - checking_object.get_position()).magnitude()
                        >= (ref_object.get_radius() + checking_object.get_radius())
                {
                    return;
                }
                let new_collision = Collision::new(
                    CollisionObjectType::Object,
                    Some(ref_object.clone()),
                    Some(checking_object.clone()),
                );
                if !object_collisions.contains(&new_collision) {
                    object_collisions.push(new_collision);
                }
            });
        }

        object_collisions.iter().for_each(|collision| {
            let (ref_id, checking_id) = (
                collision.get_primary().unwrap().get_id(),
                collision.get_secondary().unwrap().get_id(),
            );
            let [[updated_ref_pos, updated_ref_vel], [updated_check_pos, updated_check_vel]] =
                collision.resolve_objects_and_return_updates();

            self.objects_hash
                .entry(ref_id)
                .and_modify(|(affected_object, _)| {
                    affected_object.update_position(updated_ref_pos);
                    affected_object.update_velocity(updated_ref_vel);
                });
            self.objects_hash
                .entry(checking_id)
                .and_modify(|(affected_object, _)| {
                    affected_object.update_position(updated_check_pos);
                    affected_object.update_velocity(updated_check_vel);
                });
        });
        if !object_collisions.is_empty() {
            did_resolve_object_collisions = true;
        }
        if did_resolve_object_collisions {
            self.recreate_objects_from_hash();
        }
    }

    pub fn update_objects(&mut self, vk_ctx: &VulkanoContext) {
        let update_shader = update_cs::load(vk_ctx.get_device().clone()).unwrap();
        let push_constants = update_cs::ComputeConstants {
            gravity: GRAVITY_ACCELERATION,
            dt: self.dt,
        };
        let object_positions_buffer = Buffer::from_iter(
            vk_ctx.get_memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.objects
                .clone()
                .iter()
                .map(|obj| obj.get_position().as_array()),
        )
        .unwrap();
        let object_velocities_buffer = Buffer::from_iter(
            vk_ctx.get_memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.objects
                .clone()
                .iter()
                .map(|obj| obj.get_velocity().as_array()),
        )
        .unwrap();
        let update_command_buffer = get_compute_command_buffer(
            vk_ctx.clone(),
            update_shader.clone(),
            vec![
                object_positions_buffer.clone(),
                object_velocities_buffer.clone(),
            ],
            Some(push_constants),
            [self.objects.len() as u32, 1, 1],
        )
        .unwrap()
        .build()
        .unwrap();

        let future = sync::now(vk_ctx.get_device().clone())
            .then_execute(vk_ctx.get_queue().clone(), update_command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let object_positions_reader = object_positions_buffer.read().unwrap();
        let object_velocities_reader = object_velocities_buffer.read().unwrap();
        for (idx, (updated_position, updated_velocity)) in std::iter::zip(
            object_positions_reader.iter(),
            object_velocities_reader.iter(),
        )
        .enumerate()
        {
            self.objects[idx].update_position(updated_position.into());
            self.objects[idx].update_velocity(updated_velocity.into());
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

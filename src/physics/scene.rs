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

use crate::renderer::vk_core::{get_compute_command_buffer, VulkanoContext};
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
            
            // layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            // layout(binding = 0, set = 0) buffer P {
            //     vec2 pos[];
            // } positions;

            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 0, set = 0) buffer J {
                vec2 join[2];
            } joined;

            // layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            // vec2 position;
            // layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            // vec2 velocity;

            void main() {
                // uint x = gl_GlobalInvocationID.x;
                // position += 
                // velocity += vec2(0., gravity * dt);
                // joined.join[0] += vec2(joined.join[1].x * dt, joined.join[1].y * dt);
                // joined.join[1] += vec2(0., gravity * dt);
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
        let update_shader = update_cs::load(vk_ctx.device.clone()).unwrap();
        let push_constants = update_cs::ComputeConstants {
            gravity: GRAVITY_ACCELERATION,
            dt: self.dt,
        };
        for object in &mut self.objects {
            let object_buffer = Buffer::from_iter(
                vk_ctx.get_memory_allocator(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                    ..Default::default()
                },
                [
                    object.get_position().as_array(),
                    object.get_velocity().as_array(),
                ],
            )
            .unwrap();
            let update_command_buffer = get_compute_command_buffer(
                vk_ctx.clone(),
                update_shader.clone(),
                object_buffer.clone(),
                Some(push_constants),
            )
            .unwrap()
            .build()
            .unwrap();

            // let future = sync::now(vk_ctx.device.clone())
            //     .then_execute(vk_ctx.get_queue().clone(), update_command_buffer)
            //     .unwrap()
            //     .then_signal_fence_and_flush()
            //     .unwrap();

            // let mut updated_position: FVec2 = FVec2::new(0., 0.);
            // let mut updated_velocity: FVec2 = FVec2::new(0., 0.);
            // let object_buffer_contents = object_buffer.read().unwrap();
            // for (idx, val) in object_buffer_contents.iter().enumerate() {
            //     match idx {
            //         0 => updated_position = FVec2::new(val[0], val[1]),
            //         1 => updated_velocity = FVec2::new(val[0], val[1]),
            //         _ => unreachable!(),
            //     }
            // }

            // object.update_velocity(updated_velocity);
            // object.update_position(updated_position);
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

use std::hash::RandomState;
use std::{collections::HashMap, sync::Arc};

use tracing::info;
use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Queue;
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::sync::{self, GpuFuture};
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};
use winit::event_loop::EventLoop;

use crate::renderer::vk_core::{RuntimeBuffers, VulkanoContext, WindowContext};
use crate::{
    renderer::{
        vk_core::{CustomVertex, WindowEventHandler},
        vk_primitives::create_memory_allocator,
        vk_procedural_functions::{Polygon, PolygonMethods},
    },
    FVec2,
};

use super::{lib::DELTA_TIME, rigidbody::RigidBody};

pub mod update_with_collision_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            layout(push_constant) uniform ComputeConstants {
                float gravity;
                float dt;
                uint num_objects;
            };
            
            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 0, set = 0) buffer P {
                vec2 p[];
            } positions;

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 1, set = 0) buffer V {
                vec2 v[];
            } velocities;

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 2, set = 0) buffer R {
                // Had to pass in [radius, 0.0] to satisfy my `get_compute_command_buffer` function
                vec2 r[];
            } radii;

            bool do_objects_collide(uint ref_object_id, uint other_object_id) {
                if (ref_object_id == other_object_id) {
                    return false;
                }

                vec2 vector_between_coms = positions.p[ref_object_id] - positions.p[other_object_id]; 
                float distance_between_coms = length(vector_between_coms);
                float radius_ref = radii.r[ref_object_id][0];
                float radius_other = radii.r[other_object_id][0];

                if (distance_between_coms < (radius_ref + radius_other)) {
                    return true;
                }
               
                return false;
            }

            void resolve_object_collision(uint object_one_id, uint object_two_id) {
                vec2 object_one_position = positions.p[object_one_id];
                vec2 object_one_velocity = velocities.v[object_one_id];
                float object_one_radius = radii.r[object_one_id][0];
                vec2 object_two_position = positions.p[object_two_id];
                vec2 object_two_velocity = velocities.v[object_two_id];
                float object_two_radius = radii.r[object_two_id][0];

                vec2 com_distance_vector = object_one_position - object_two_position;
                float overlapping_distance = length(com_distance_vector) - object_one_radius - object_two_radius;
                float inclination_angle_of_vector = atan(com_distance_vector.y, com_distance_vector.x);
                vec2 distance_to_move_either =  vec2(overlapping_distance * cos(inclination_angle_of_vector), overlapping_distance * sin(inclination_angle_of_vector));

                vec2 updated_velocity_one = object_one_velocity - (dot(object_one_velocity - object_two_velocity, object_one_position - object_two_position))
                / (pow(length(object_two_position - object_one_position), 2)) * (object_one_position - object_two_position);
                vec2 updated_velocity_two = object_two_velocity - (dot(object_two_velocity - object_one_velocity, object_two_position - object_one_position))
                / (pow(length(object_two_position - object_one_position), 2)) * (object_two_position - object_one_position);


                positions.p[object_one_id] += -1. * distance_to_move_either;
                positions.p[object_two_id] += 1. * distance_to_move_either;
                velocities.v[object_one_id] = updated_velocity_one;
                velocities.v[object_two_id] = updated_velocity_two;
            }

            void check_and_resolve_world_collision(uint object_id) {
                vec2 object_position = positions.p[object_id];
                float object_radius = radii.r[object_id][0];
                bool crossed_lateral = false;
                bool crossed_vertical = false;

                if ((abs(object_position[0]) + abs(object_radius)) > 1.) {
                    crossed_lateral = true;
                }
                if ((abs(object_position[1]) + abs(object_radius)) > 1.) {
                    crossed_vertical = true;
                }

                if (crossed_lateral) {
                    velocities.v[object_id].x *= -1.;
                }
                if (crossed_vertical) {
                    velocities.v[object_id].y *= -1.;
                }
            }

            void main() {
                uint x = gl_GlobalInvocationID.x;

                // Check and resolve object-world collisions.
                check_and_resolve_world_collision(x);

                // Check and resolve object-object collisions.
                for (uint other_idx = 0; other_idx < num_objects; other_idx++) {
                    if (x == other_idx) {
                        continue;
                    };

                    bool collides = do_objects_collide(x, other_idx);
                    if (collides) {
                        resolve_object_collision(x, other_idx);
                    }
                }

                // Update state as usual. First-order Euler, or something.
                vec2 position_change = vec2(velocities.v[x] * dt);
                vec2 velocity_change = vec2(0, gravity * dt);
                positions.p[x] += position_change;
                velocities.v[x] += velocity_change;
            }
            ",
    }
}

pub struct Scene {
    pub objects: Vec<RigidBody>,
    pub objects_hash: HashMap<u8, (RigidBody, Polygon)>,
    pub dt: f32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
    pub fn with_objects(mut objects: Vec<RigidBody>) -> Self {
        // NOTE: Correcting for the vulkan coordinate system: resetting to (0,0) being lower left.
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

    pub fn return_compute_shader_buffers(
        &self,
        memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    ) -> RuntimeBuffers {
        let objects_positions = Buffer::from_iter(
            memory_allocator.clone(),
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
        let objects_velocities = Buffer::from_iter(
            memory_allocator.clone(),
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
        let objects_radii = Buffer::from_iter(
            memory_allocator,
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
                .map(|obj| [obj.get_radius(), 0.]),
        )
        .unwrap();

        RuntimeBuffers {
            objects_positions,
            objects_velocities,
            objects_radii,
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
        let window_ctx = WindowContext::new(&event_loop);
        let vk_ctx = VulkanoContext::with_window_context(&window_ctx, &event_loop);
        let window_ctx_handler = WindowEventHandler::new(
            &event_loop,
            self.return_compute_shader_buffers(vk_ctx.get_memory_allocator()),
            vk_ctx,
            window_ctx,
            self.objects.len() as u32,
        );
        window_ctx_handler.run_with_scene(self, event_loop);
    }

    // pub fn check_and_world_resolve_collisions(&mut self) {
    // Checking for object-world collisions
    // for object in &mut self.objects {
    //     let world_collisions: Option<Collision> = object.check_world_collisions();
    //     if world_collisions.is_none() {
    //         continue;
    //     }
    //     if let CollisionObjectType::World(boundary_info) =
    //         world_collisions.unwrap().get_collision_type()
    //     {
    //         object.resolve_world_collision(boundary_info);
    //     }
    // }
    // self.recreate_hash();

    // let mut did_resolve_object_collisions: bool = false;
    // // Checking for object-object collisions
    // let mut object_collisions: Vec<Collision> = Vec::new();
    // for ref_object in &self.objects {
    //     self.objects.iter().for_each(|checking_object| {
    //         if /* skip checking itself */ checking_object == ref_object
    //             || /* no detected collision */ (ref_object.get_position() - checking_object.get_position()).magnitude()
    //                 >= (ref_object.get_radius() + checking_object.get_radius())
    //         {
    //             return;
    //         }
    //         let new_collision = Collision::new(
    //             CollisionObjectType::Object,
    //             Some(ref_object.clone()),
    //             Some(checking_object.clone()),
    //         );
    //         if !object_collisions.contains(&new_collision) {
    //             object_collisions.push(new_collision);
    //         }
    //     });
    // }

    // object_collisions.iter().for_each(|collision| {
    //     let (ref_id, checking_id) = (
    //         collision.get_primary().unwrap().get_id(),
    //         collision.get_secondary().unwrap().get_id(),
    //     );
    //     let [[updated_ref_pos, updated_ref_vel], [updated_check_pos, updated_check_vel]] =
    //         collision.resolve_objects_and_return_updates();

    //     self.objects_hash
    //         .entry(ref_id)
    //         .and_modify(|(affected_object, _)| {
    //             affected_object.update_position(updated_ref_pos);
    //             affected_object.update_velocity(updated_ref_vel);
    //         });
    //     self.objects_hash
    //         .entry(checking_id)
    //         .and_modify(|(affected_object, _)| {
    //             affected_object.update_position(updated_check_pos);
    //             affected_object.update_velocity(updated_check_vel);
    //         });
    // });
    // if !object_collisions.is_empty() {
    //     did_resolve_object_collisions = true;
    // }
    // if did_resolve_object_collisions {
    //     self.recreate_objects_from_hash();
    // }
    // }

    pub fn update_with_buffers(
        &mut self,
        device: Arc<Device>,
        queue: Arc<Queue>,
        compute_command_buffer: Arc<PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>>,
        runtime_buffers: RuntimeBuffers,
    ) {
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), compute_command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let binding = runtime_buffers.objects_positions.clone();
        let object_positions_reader = binding.read().unwrap();
        let binding = runtime_buffers.objects_velocities.clone();
        let object_velocities_reader = binding.read().unwrap();
        for (idx, (updated_position, updated_velocity)) in std::iter::zip(
            object_positions_reader.iter(),
            object_velocities_reader.iter(),
        )
        .enumerate()
        {
            self.objects[idx].update_position(updated_position.into());
            self.objects[idx].update_velocity(updated_velocity.into());
        }
        // self.check_and_world_resolve_collisions();
        self.recreate_hash();

        info!("{:?}", {
            self.objects
                .clone()
                .iter()
                .map(|obj| {
                    format!(
                        "id = {:?} with velocity = {:?}",
                        obj.get_id(),
                        obj.get_velocity()
                    )
                })
                .collect::<Vec<String>>()
        });
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

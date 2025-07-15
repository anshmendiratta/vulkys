use std::{collections::HashMap, sync::Arc};

use nalgebra::vector;
use rapier2d::prelude::{ColliderSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};
use winit::event_loop::EventLoop;

use crate::vulkan::{
    contexts::{VulkanoContext, WindowContext},
    core::{CustomVertex, WindowEventHandler},
    procedural::Polygon,
};

use super::rigidbody::{convert_rigidbody_to_collider_builder, RigidBody};

#[derive(Clone)]
pub struct SceneInfo {
    pub objects: Vec<RigidBody>,
    pub dt: f32,
    pub gravity: f32,
}

#[allow(dead_code)]
pub struct Scene {
    pub polygon_set: HashMap<RigidBodyHandle, Polygon>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: f32,
    dt: f32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
    pub fn with_info(scene_info: SceneInfo) -> Self {
        let mut objects_map: HashMap<RigidBodyHandle, Polygon> = HashMap::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        for rb in scene_info.objects {
            let rb_position = rb.get_position();
            let polygon = rb.to_polygon();
            let cb = convert_rigidbody_to_collider_builder(rb).build();
            let rbb =
                RigidBodyBuilder::dynamic().translation(vector![rb_position.x, rb_position.y]);
            let handle_index = rigid_body_set.insert(rbb);

            objects_map.insert(handle_index, polygon);
            collider_set.insert_with_parent(cb, handle_index, &mut rigid_body_set);
        }
        dbg!(collider_set.len(), rigid_body_set.len());

        Self {
            dt: scene_info.dt,
            polygon_set: objects_map,
            rigid_body_set,
            collider_set: ColliderSet::new(),
            gravity: scene_info.gravity,
        }
    }

    pub fn return_objects_as_vertex_buffer(
        &self,
        allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    ) -> Subbuffer<[CustomVertex]> {
        let vertex_buffer_data = {
            let mut buffer_data: Vec<CustomVertex> = Vec::with_capacity(self.polygon_set.len() * 3);
            for (_, polygon) in &self.polygon_set {
                buffer_data = [buffer_data, polygon.clone().into_flattened()].concat();
            }
            buffer_data
        };
        Buffer::from_iter(
            allocator,
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
        let window_ctx_handler = WindowEventHandler::new(&event_loop, vk_ctx, window_ctx);
        window_ctx_handler.run_with_scene(self, event_loop);
    }

    // pub fn recreate_hash_from_objects(&mut self) {
    //     let polygons: Vec<Polygon> = self.objects.iter().map(|body| body.to_polygon()).collect();

    //     let mut objects_as_hash: HashMap<u8, (RigidBody, Polygon)> =
    //         HashMap::with_capacity_and_hasher(self.objects.len(), RandomState::new());
    //     for (rigidbody, polygon) in std::iter::zip(&self.objects, polygons) {
    //         objects_as_hash.insert(rigidbody.get_id(), (rigidbody.clone(), polygon));
    //     }

    //     self.objects_map = objects_as_hash;
    // }

    // pub fn recreate_objects_from_hash(&mut self) {
    //     self.objects.clear();
    //     for (rigidbody, _) in self.objects_map.values() {
    //         self.objects.push(rigidbody.clone());
    //     }
    // }
}

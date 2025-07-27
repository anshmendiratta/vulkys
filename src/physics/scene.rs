use std::{collections::HashMap, sync::Arc};

use ecolor::Color32;
use nalgebra::vector;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, ColliderSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};
use winit::event_loop::EventLoop;

use crate::vulkan::procedural::generate_polygon_triangles;
use crate::vulkan::{
    contexts::{VulkanoContext, WindowContext},
    core::{CustomVertex, WindowEventHandler},
    procedural::Polygon,
};
use crate::FVec2;

use super::rigidbody::{convert_rigidbody_to_collider_builder, RigidBody};

#[derive(Clone)]
pub struct SceneInfo {
    pub objects: Vec<RigidBody>,
    pub dt: f32,
    pub gravity: f32,
}

#[allow(dead_code)]
pub struct Scene {
    index_map: HashMap<u128, (RigidBodyHandle, ColliderHandle)>,
    pub polygon_set: HashMap<u128, (Polygon, Color32)>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: f32,
    dt: f32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
    pub fn with_info(scene_info: SceneInfo) -> Self {
        let mut index_map = HashMap::new();
        let mut polygon_set: HashMap<u128, (Polygon, Color32)> = HashMap::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        for (i, rb) in scene_info.objects.iter().enumerate() {
            let rb_position = rb.get_position();
            let rb_velocity = rb.get_velocity();
            let polygon = rb.to_polygon();
            let color = rb.get_color();
            let cb = convert_rigidbody_to_collider_builder(rb.clone()).build();
            let rbb = RigidBodyBuilder::dynamic()
                .translation(vector![rb_position.x, rb_position.y])
                .linvel(vector![rb_velocity.x, rb_velocity.y]);
            let rb_handle = rigid_body_set.insert(rbb);
            let cb_handle = collider_set.insert_with_parent(cb, rb_handle, &mut rigid_body_set);

            polygon_set.insert(i as u128, (polygon, color));
            index_map.insert(i as u128, (rb_handle, cb_handle));
        }

        // Add world colliders.
        let pos_x_world_rigidbody = RigidBodyBuilder::fixed()
            .translation(vector![2., 0.])
            .build();
        let pos_x_world_collider = ColliderBuilder::cuboid(1., 1.).build();
        let neg_x_world_rigidbody = RigidBodyBuilder::fixed()
            .translation(vector![-2., 0.])
            .build();
        let neg_x_world_collider = ColliderBuilder::cuboid(1., 1.).build();
        let pos_y_world_rigidbody = RigidBodyBuilder::fixed()
            .translation(vector![0., 2.])
            .build();
        let pos_y_world_collider = ColliderBuilder::cuboid(1., 1.).build();
        let neg_y_world_rigidbody = RigidBodyBuilder::fixed()
            .translation(vector![0., -2.])
            .build();
        let neg_y_world_collider = ColliderBuilder::cuboid(1., 1.).build();

        // Discard handles because they will not be referenced.
        let mut pxwrb_handle = rigid_body_set.insert(pos_x_world_rigidbody);
        let _ = collider_set.insert_with_parent(
            pos_x_world_collider,
            pxwrb_handle,
            &mut rigid_body_set,
        );
        pxwrb_handle = rigid_body_set.insert(neg_x_world_rigidbody);
        let _ = collider_set.insert_with_parent(
            neg_x_world_collider,
            pxwrb_handle,
            &mut rigid_body_set,
        );
        pxwrb_handle = rigid_body_set.insert(pos_y_world_rigidbody);
        let _ = collider_set.insert_with_parent(
            pos_y_world_collider,
            pxwrb_handle,
            &mut rigid_body_set,
        );
        pxwrb_handle = rigid_body_set.insert(neg_y_world_rigidbody);
        let _ = collider_set.insert_with_parent(
            neg_y_world_collider,
            pxwrb_handle,
            &mut rigid_body_set,
        );

        Self {
            index_map,
            dt: scene_info.dt,
            polygon_set,
            rigid_body_set,
            collider_set,
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
                buffer_data = [buffer_data, polygon.clone().0.into_flattened()].concat();
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

    pub fn update_polygon_set(&mut self) {
        for (i, (rb_handle, cb_handle)) in self.index_map.iter() {
            let rigid_body = self.rigid_body_set.get(*rb_handle).unwrap();
            let collider_body = self.collider_set.get(*cb_handle).unwrap();
            let (x, y) = (
                rigid_body.position().translation.vector[0],
                rigid_body.position().translation.vector[1],
            );
            let color = self.polygon_set[i].1;
            let radius = collider_body.shape().as_ball().unwrap().radius;
            // NOTE: Temporary.
            let vertex_count = 32; // Circle.
            let polygon: Vec<[CustomVertex; 3]> = generate_polygon_triangles(
                vertex_count,
                FVec2::new(x, y).to_custom_vertex(Some(color)),
                radius,
                color,
            );

            // Update map.
            self.polygon_set.entry(*i).and_modify(move |p| {
                *p = (polygon, color);
            });
        }
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

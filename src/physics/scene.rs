use std::{collections::HashMap, sync::Arc};

use crate::vulkan::primitives::CustomVertex;
use nalgebra::vector;
use rapier3d::prelude::{
    ColliderBuilder, ColliderHandle, ColliderSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use vulkano::buffer::{Buffer, IndexBuffer, Subbuffer};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

use super::rigidbody::RigidBody;

#[derive(Clone)]
pub struct SceneInfo {
    pub objects: Vec<RigidBody>,
    pub dt: f32,
    pub gravity: f32,
}

#[allow(dead_code)]
pub struct Scene {
    index_map: HashMap<u128, (RigidBodyHandle, ColliderHandle)>,
    pub object_set: HashMap<u128, RigidBody>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: f32,
    dt: f32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
    pub fn with_info(scene_info: SceneInfo) -> Self {
        let mut index_map = HashMap::new();
        let mut object_set: HashMap<u128, RigidBody> = HashMap::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        for (i, rb) in scene_info.objects.iter().enumerate() {
            let rb_position = rb.get_init_position();
            let rb_velocity = rb.get_init_velocity();
            // let color = rb.get_color();
            let cb = rb.convert_to_collider().build();
            let rbb = RigidBodyBuilder::dynamic()
                .translation(vector![rb_position.x, rb_position.y, rb_position.z])
                .linvel(vector![rb_velocity.x, rb_velocity.y, rb_velocity.z]);
            let rb_handle = rigid_body_set.insert(rbb);
            let cb_handle = collider_set.insert_with_parent(cb, rb_handle, &mut rigid_body_set);

            object_set.insert(i as u128, rb.clone());
            index_map.insert(i as u128, (rb_handle, cb_handle));
        }

        // FIX: Correct world colliders for 3D. Namely, remove the ceiling collider and add x/z colliders with a negative y collider.
        // FIX: Also fix translations.
        // Add world colliders.
        let floor_rb = RigidBodyBuilder::fixed()
            .translation(vector![0., -1., 0.])
            .build();
        let floor_cb = ColliderBuilder::cuboid(100., 0.5, 100.).build();

        let floor_rb_handle = rigid_body_set.insert(floor_rb);
        // Discard handle because it will not be referenced.
        let _ = collider_set.insert_with_parent(floor_cb, floor_rb_handle, &mut rigid_body_set);

        Self {
            index_map,
            dt: scene_info.dt,
            object_set,
            rigid_body_set,
            collider_set,
            gravity: scene_info.gravity,
        }
    }

    pub fn return_vertex_buffer(
        &self,
        allocator: Arc<StandardMemoryAllocator>,
    ) -> Subbuffer<[CustomVertex]> {
        let vertex_buffer_data = {
            let mut buffer_data: Vec<CustomVertex> = Vec::new();
            for (i, _) in self.object_set.iter() {
                let rigid_body = self.object_set.get(i).unwrap();
                let (rb_handle, _) = self.index_map.get(i).unwrap();
                let rapier_rigid_body = self.rigid_body_set.get(*rb_handle).unwrap();
                let _translation = rapier_rigid_body.translation();
                let rotation = rapier_rigid_body.rotation().to_rotation_matrix();
                buffer_data = [
                    buffer_data,
                    rigid_body.get_vertices(rotation, rigid_body.get_init_position()),
                ]
                .concat();
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

    pub fn return_index_buffer(&self, allocator: Arc<StandardMemoryAllocator>) -> IndexBuffer {
        let index_buffer_data = {
            let mut buffer_data: Vec<u16> = Vec::new();
            for (i, _) in self.object_set.iter() {
                let rigid_body = self.object_set.get(i).unwrap();
                buffer_data = [buffer_data, rigid_body.get_vertex_indices()].concat();
            }
            buffer_data
        };

        let index_subbuffer = Buffer::from_iter(
            allocator,
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            index_buffer_data.clone(),
        )
        .expect("scene: could not produce index buffer from objects");

        // dbg!(&index_buffer_data, index_buffer_data.len());

        IndexBuffer::U16(index_subbuffer)
    }
}

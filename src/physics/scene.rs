use tracing::info;

use crate::{renderer::vk_core::WindowEventHandler, FVec2};

use super::{
    lib::{DELTA_TIME, GRAVITY_ACCELERATION},
    rigidbody::RigidBody,
};

pub struct Scene {
    objects: Vec<RigidBody>,
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

        Self {
            objects,
            dt: DELTA_TIME,
        }
    }
    // TODO: modify the `run_with_objects` to loop over itself with updated values instaed of being called in a loop
    // Currently runs the window context handler function over and over again.
    // That is BAD.
    pub fn run(mut self) {
        loop {
            let windowcx_handler = WindowEventHandler::new();
            windowcx_handler.run_with_objects(self.objects.clone());
            self.update_objects();
            info!("Running main scene");
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
    }
}

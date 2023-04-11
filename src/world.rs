pub mod rigidbodyobjects {
    // mod rigidbody;
}

mod world {
    pub struct World {
        gravity: f32, 
        objects: Vec<RigidBody>,
        restitution: f32,
    }

    pub struct Plane {
        y: f32,
        angle: f32,
    }

    impl World {
        pub fn add(&self, object: RigidBody) {
            &self.objects.push(object)
        }

        pub fn get_gravity(self) -> &'static f32 {
            &self.gravity
        }

        pub fn get_objects(&self) -> Vec<RigidBody> {
            &self.objects
        }

        pub fn get_restitution(&self) -> f32 {
            &self.restitution
        }

        pub fn step(&self, dt: f32) {
            calculate_state(dt)
        }
    }
}
pub struct Collision {
    axis: &str.to_lower(),
    object: RigidBody,
    time: f32,
}

impl Collision {
    pub fn get_time(&self) -> f32 {
        &self.time
    }

    pub fn get_axis(&self) -> &str {
        &self.axis
    }

    pub fn get_object(&self) -> RigidBody {
        &self.object
    }
}

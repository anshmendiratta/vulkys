use nalgebra::{Matrix4, Vector3};

static CAMERA_LOOK_AT: Vector3<f32> = Vector3::new(0., 0., 0.);
static WORLD_UP: Vector3<f32> = Vector3::new(0., -1., 0.);
pub static mut CAMERA: Camera = Camera {
    position: Vector3::new(3.464, 0.9553, 0.7854), // Corresponding to (x, y, z) = (2, 2, 2).
};

/// `position` is a vector of (r, theta, phi). That is, the positionn fo the camera in spherical coordinates.
pub struct Camera {
    position: Vector3<f32>,
}

impl Camera {
    pub fn to_view_matrix(&self) -> Matrix4<f32> {
        glm::look_at(&self.position_as_cartesian(), &CAMERA_LOOK_AT, &WORLD_UP)
    }

    pub fn position_as_cartesian(&self) -> Vector3<f32> {
        let x = self.position.x * self.position.z.sin() * self.position.y.cos(); // r * sin(phi) * cos(theta)
        let y = self.position.x * self.position.z.sin() * self.position.y.sin(); // r * sin(phi) * sin(theta)
        let z = self.position.x * self.position.z.cos(); // r * cos(phi)

        Vector3::new(x, y, z)
    }

    pub fn increment_r(&mut self) {
        self.position.x += 0.1
    }

    pub fn increment_theta(&mut self) {
        self.position.y += 0.1
    }

    pub fn increment_phi(&mut self) {
        self.position.z += 0.1
    }

    pub fn decrement_r(&mut self) {
        self.position.x -= 0.1
    }

    pub fn decrement_theta(&mut self) {
        self.position.y -= 0.1
    }

    pub fn decrement_phi(&mut self) {
        self.position.z -= 0.1
    }
}

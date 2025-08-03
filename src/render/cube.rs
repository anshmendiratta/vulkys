use glm::Vec3;

// Unit cube.
pub static CUBE_VERTICES: [Vec3; 8] = [
    Vec3::new(0.5, 0.5, 0.5),    // right    top  back
    Vec3::new(-0.5, 0.5, 0.5),   //  left    top  back
    Vec3::new(0.5, -0.5, 0.5),   // right bottom  back
    Vec3::new(0.5, 0.5, -0.5),   // right    top front
    Vec3::new(-0.5, -0.5, 0.5),  //  left bottom  back
    Vec3::new(0.5, -0.5, -0.5),  // right bottom front
    Vec3::new(-0.5, 0.5, -0.5),  //  left    top front
    Vec3::new(-0.5, -0.5, -0.5), //  left bottom front
];

pub static CUBE_INDICES: [u16; 36] = [
    0, 1, 2, 2, 4, 1, // back face
    3, 6, 5, 5, 7, 6, // front face
    0, 1, 3, 3, 6, 1, // top face
    2, 4, 5, 5, 7, 4, // bottom face
    1, 6, 4, 4, 7, 6, // left face
    0, 3, 2, 2, 5, 3, // right face
];

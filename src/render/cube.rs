use glm::Vec3;

pub static CUBE_VERTICES: [Vec3; 8] = [
    Vec3::new(1.0, 1.0, 1.0),    // right    top  back
    Vec3::new(-1.0, 1.0, 1.0),   //  left    top  back
    Vec3::new(1.0, -1.0, 1.0),   // right bottom  back
    Vec3::new(1.0, 1.0, -1.0),   // right    top front
    Vec3::new(-1.0, -1.0, 1.0),  //  left bottom  back
    Vec3::new(1.0, -1.0, -1.0),  // right bottom front
    Vec3::new(-1.0, 1.0, -1.0),  //  left    top front
    Vec3::new(-1.0, -1.0, -1.0), //  left bottom front
];

pub static CUBE_INDICES: [usize; 36] = [
    0, 1, 2, 2, 4, 1, // back face
    3, 6, 5, 5, 7, 6, // front face
    0, 1, 3, 3, 6, 1, // top face
    2, 4, 5, 5, 7, 4, // bottom face
    1, 6, 4, 4, 7, 6, // left face
    0, 3, 2, 2, 5, 3, // right face
];

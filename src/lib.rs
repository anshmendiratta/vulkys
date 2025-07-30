extern crate nalgebra_glm as glm;

pub mod core;
pub mod gui;
pub mod physics;
pub mod vulkan;

const WINDOW_LENGTH: f32 = 1000.;

// #[derive(Clone, Copy, Debug, PartialEq, BufferContents)]
// #[repr(C)]
// pub struct FVec3 {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl FVec3 {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Self { x, y, z }
//     }
//     pub fn to_custom_vertex(&self, color: Option<Color32>) -> CustomVertex {
//         CustomVertex {
//             position: Vec3::new(self.x, self.y, self.z),
//             color: if let Some(color) = color {
//                 color.to_array()
//             } else {
//                 [0, 0, 0, 0]
//             },
//         }
//     }
//     pub fn as_array(&self) -> [f32; 2] {
//         [self.x, self.y]
//     }
//     pub fn scale(&mut self, scale_factor: f32) -> Self {
//         Self {
//             x: self.x * scale_factor,
//             y: self.y * scale_factor,
//             z: self.z * scale_factor,
//         }
//     }
//     pub fn magnitude(&self) -> f32 {
//         (self.x * self.x + self.y * self.y).powf(0.5)
//     }
//     pub fn get_unit(&self) -> Self {
//         let magnitude = self.magnitude();
//         Self {
//             x: self.x / magnitude,
//             y: self.y / magnitude,
//             z: self.z / magnitude,
//         }
//     }
//     pub fn dot(&self, other: FVec3) -> f32 {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }
//     pub fn get_polar_angle(&self) -> f32 {
//         atan2f(self.y, self.x)
//     }
// }

// impl From<&[f32; 3]> for FVec3 {
//     fn from(value: &[f32; 3]) -> Self {
//         FVec3 {
//             x: value[0],
//             y: value[1],
//             z: value[2],
//         }
//     }
// }

// impl Mul<f32> for FVec3 {
//     type Output = Self;

//     fn mul(self, scalar: f32) -> Self::Output {
//         Self {
//             x: self.x * scalar,
//             y: self.y * scalar,
//             z: self.z * scalar,
//         }
//     }
// }

// impl Sub for FVec3 {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }

// impl Add for FVec3 {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         FVec3 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

// impl AddAssign for FVec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x = self.x + rhs.x;
//         self.y = self.y + rhs.y;
//     }
// }

// impl std::fmt::Display for FVec3 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let display_string = &format!("({}, {})", self.x, self.y)[..];
//         f.write_str(display_string)
//     }
// }

use std::ops::Range;

pub const DELTA_TIME: f32 = 0.001;
#[doc = "Positive to align with vulkan's coordinate system"]
pub const GRAVITY_ACCELERATION: f32 = 18.;
pub const WORLD_BOUNDS: (Range<f32>, Range<f32>) = ((-1 as f32..1 as f32), (-1 as f32..1 as f32));
pub const COEFF_RESTITUTION: f32 = 0.9;

pub(crate) use crate::physics::rigidbodies::Updateable;

use crate::physics::step::world_step;

pub(crate) struct World {
    pub gravity: (f64, f64),
    pub objects: Vec<Box<dyn Updateable>>,
    pub restitution: f64,
    pub boundary: Boundary,
    pub time: f64,
    pub dt: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            gravity: (0.0, -9.81),
            objects: Vec::new(),
            restitution: 1.0,
            boundary: Boundary {
                x_range: (-1.0, 1.0),
                y_range: (-1.0, 1.0),
            },
            time: 0.0,
            dt: 0.1,
        }
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Updateable + AsRef<T> + 'static,
    {
        self.objects.push(Box::new(object))
    }

    pub fn get_gravity(&self) -> (f64, f64) {
        self.gravity
    }

    pub fn get_objects(self) -> Vec<Box<dyn Updateable>> {
        self.objects
    }

    pub fn get_restitution(&self) -> f64 {
        self.restitution
    }

    pub fn get_timestep(&self) -> f64 {
        self.dt
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn world_step<T>(self)
    where
        T: Updateable + AsRef<T> + Clone,
    {
        world_step(self)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Boundary {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
}

impl Boundary {
    pub fn new() -> Self {
        Self {
            x_range: (-1.0, 1.0),
            y_range: (-1.0, 1.0),
        }
    }

    pub fn get_x_range(&self) -> (f64, f64) {
        self.x_range
    }

    pub fn get_y_range(&self) -> (f64, f64) {
        self.y_range
    }
}

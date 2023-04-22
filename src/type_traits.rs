use crate::rigidbodies::*;

type Radians = f64;

pub trait VectorMethods {
    fn scale_vector(&self, scalar: f64) -> Vec<f64>;
    fn rotate_vector(&mut self, angle: Radians) -> Vec<f64>;
    fn magnitude(&self) -> f64;
}

impl VectorMethods for Vec<f64> {
    fn scale_vector(&self, scalar: f64) -> Vec<f64> {
        let result: Vec<f64> = [scalar * &self[0], scalar * &self[1]].to_vec();
        result
    }

    fn rotate_vector(&mut self, angle: Radians) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::with_capacity(2);
        result[0] = &self[0] * angle.cos() - &self[1] * angle.sin();
        result[1] = &self[0] * angle.sin() + &self[1] * angle.cos();
        result
    }

    fn magnitude(&self) -> f64 {
        let mut square_sum: f64 = 0.0;
        for basis in *&self {
            square_sum += basis.powf(2.0)
        }
        square_sum.powf(0.5)
    }
}

impl VectorMethods for &Vec<f64> {
    fn scale_vector(&self, scalar: f64) -> Vec<f64> {
        let result: Vec<f64> = [scalar * &self[0], scalar * &self[1]].to_vec();
        result
    }

    fn rotate_vector(&mut self, angle: Radians) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::with_capacity(2);
        result[0] = &self[0] * angle.cos() - &self[1] * angle.sin();
        result[1] = &self[0] * angle.sin() + &self[1] * angle.cos();
        result
    }

    fn magnitude(&self) -> f64 {
        let mut square_sum: f64 = 0.0;
        for basis in *self {
            square_sum += basis.powf(2.0)
        }
        square_sum.powf(0.5)
    }
}

pub trait MetaMethods {
    fn to_string(&self) -> String
    where
        Self: std::fmt::Debug,
    {
        let mut raw_string = String::from(format!("{:?}", self));
        let idx: Option<usize> =
            raw_string.find(|x: char| x.to_string() == "(" || x.to_string() == "{");
        match idx {
            Some(T) => raw_string.truncate(T),
            None => (),
        }
        raw_string
    }
}

impl<T> HandleData for Box<T> {
    fn get_mass(&self) -> f64 {
        0.0
    }

    fn get_position(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
    fn set_position(&mut self, _new_position: (f64, f64)) {
    }

    fn get_velocity(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
    fn set_velocity(&mut self, _new_velocity: (f64, f64)) {        
    }

    fn get_angular_velocity(&self) -> f64 {
        0.0
    }
    fn set_angular_velocity(&mut self, _new_angular_velocity: f64) {
    }

    fn get_acceleration(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
    fn set_acceleration(&mut self, _new_acceleration: (f64, f64)) {
    }
}

impl<T: Updateable> Updateable for Box<T> {
    fn get_rigidbody(&self) -> RigidBody {
        RigidBody { position: (0.0, 0.0), velocity: (0.0, 0.0), mass: 0.0 }
    }
}

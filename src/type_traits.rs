use crate::rigidbodies::*;

type Radians = f64;

/// Auxiliary methods for mathematical vectors.
pub trait VectorMethods {
    fn scale_vector(&self, scalar: f64) -> Vec<f64>;
    fn rotate_vector(&mut self, angle: Radians) -> Vec<f64>;
    // Also equivalent to the 'norm' here.
    fn magnitude(&self) -> f64;
}

/// Specific implement of the above methods for vectors containing floating points (64 bits).
impl VectorMethods for Vec<f64> {
    fn scale_vector(&self, scalar: f64) -> Vec<f64> {
        let result: Vec<f64> = [scalar * &self[0], scalar * &self[1]].to_vec();
        result
    }

    fn rotate_vector(&mut self, angle: Radians) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::with_capacity(2);
        // Standard result of multiplying a vector with a rotation matrix in two dimensions.
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

/// Same implementation but for references to floating-point vector.
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

/// Defines a method to print the identifier of a Struct as a String.
pub trait MetaMethods {
    fn to_string(&self) -> String
    where
        Self: std::fmt::Debug,
    {
        let mut raw_string = String::from(format!("{:?}", self));
        // idx stands for Index. The below line finds the first occurence of a "(" or "{" and stops there. 
        let idx: Option<usize> =
            raw_string.find(|x: char| x.to_string() == "(" || x.to_string() == "{");
        // If the above line is successful and returns a number (of type usize), then the string is truncated at that point, leaving just the Struct name. Otherwise, if no index is found, the the string is presumed to already have isolated the Struct name.
        match idx {
            Some(T) => raw_string.truncate(T),
            None => (),
        }
        raw_string
    }
}

/// A debugging implementation meant to work around the constraints imposed by some functions (like when they pass the generic parameter `T` with `Updateable`).
/// Uses dummy numbers
impl<T> HandleData for Box<T> {
    fn get_mass(&self) -> f64 {
        0.0
    }

    fn get_radius(&self) -> f64 {
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

/// A debugging implementation meant to work around the constraints imposed by some functions (like when they pass the generic parameter `T` with `Updateable`).
/// Uses dummy numbers
impl<T: Updateable> Updateable for Box<T> {
    fn get_rigidbody(&self) -> RigidBody {
        RigidBody { position: (0.0, 0.0), velocity: (0.0, 0.0), mass: 0.0, radius: 1.0 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_struct_name() {
        #[derive(Debug)]
        struct NewStructOne(usize);
        impl MetaMethods for NewStructOne {}

        let s = NewStructOne(2);
        assert_eq!(String::from("NewStructOne"), s.to_string())
    }
}


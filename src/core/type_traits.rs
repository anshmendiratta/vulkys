type Radians = f64;

pub trait PhysicsVectorMethods {
    fn scale_vector(&self, scalar: f64) -> (f64, f64);
    fn rotate_vector(&mut self, angle: Radians) -> (f64, f64);
    fn magnitude(&self) -> f64;
}

// impl PhysicsVectorMethods for (f64, f64) {
//     fn scale_vector(&self, scalar: f64) -> (f64, f64) {
//         (scalar * self.0, scalar * self.1)
//     }
//     fn rotate_vector(&mut self, angle: Radians) -> (f64, f64) {
//         (
//             self.0 * angle.cos() - self.1 * angle.sin(),
//             self.0 * angle.sin() + self.1 * angle.cos(),
//         )
//     }
//     fn magnitude(&self) -> f64 {
//         let mut square_sum: f64 = 0.0;
//         match self {
//             (first, second) => {
//                 square_sum += first.powf(2.0);
//                 square_sum += second.powf(2.0);
//             }
//         }
//         square_sum.powf(0.5)
//     }
// }

impl PhysicsVectorMethods for &(f64, f64) {
    fn scale_vector(&self, scalar: f64) -> (f64, f64) {
        (scalar * self.0, scalar * self.1)
    }
    fn rotate_vector(&mut self, angle: Radians) -> (f64, f64) {
        (
            self.0 * angle.cos() - self.1 * angle.sin(),
            self.0 * angle.sin() + self.1 * angle.cos(),
        )
    }
    fn magnitude(&self) -> f64 {
        let mut square_sum: f64 = 0.0;
        match self {
            (first, second) => {
                square_sum += first.powf(2.0);
                square_sum += second.powf(2.0);
            }
        }
        square_sum.powf(0.5)
    }
}

pub trait MetaMethods {
    fn to_string(&self) -> String
    where
        Self: std::fmt::Debug,
    {
        let mut raw_string = format!("{:?}", self);
        let idx: Option<usize> =
            raw_string.find(|x: char| x.to_string() == "(" || x.to_string() == "{");

        if let Some(t) = idx {
            raw_string.truncate(t)
        }
        raw_string
    }
}

// impl<T> HandleData for Box<T> {
//     fn set_position(&mut self, _new_position: (f64, f64)) {
//
//     }
//     fn get_position(&self) -> (f64, f64) {
//         (0.0, 0.0)
//     }
//     fn set_velocity(&mut self, _new_velocity: (f64, f64)) {}
//     fn get_velocity(&self) -> (f64, f64) {
//         (0.0, 0.0)
//     }
//     fn set_angular_velocity(&mut self, _new_angular_velocity: f64) {}
//     fn get_angular_velocity(&self) -> f64 {
//         0.0
//     }
//     fn set_acceleration(&mut self, _new_acceleration: (f64, f64)) {}
//     fn get_acceleration(&self) -> (f64, f64) {
//         (0.0, 0.0)
//     }
//     fn get_object_name(&self) -> &str {
//         todo!()
//     }
//
//     fn get_radius(&self) -> f64 {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_struct_name() {
        #[derive(Debug)]
        struct NewStructOne();
        impl MetaMethods for NewStructOne {}

        let s = NewStructOne();
        assert_eq!(String::from("NewStructOne"), s.to_string())
    }
}

type Radians = f64;

trait VectorMethods {
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

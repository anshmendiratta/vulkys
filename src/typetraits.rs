trait Norm {
    fn norm(&self) -> f64;
}

impl Norm for Vec<f64> {
    fn norm(&self) -> f64 {
        let mut square_sum: f64 = 0.0;
        for basis in *&self {
            square_sum += basis.powf(2.0)
        }

        square_sum.powf(0.5)
    }
}
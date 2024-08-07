use crate::core::type_traits::PhysicsVectorMethods;
use libm::acos;

fn add(t: &(f64, f64), j: &(f64, f64)) -> (f64, f64) {
    (t.0 - j.0, t.1 - j.1)
}

fn subtract(t: &(f64, f64), j: &(f64, f64)) -> (f64, f64) {
    (t.0 - j.0, t.1 - j.1)
}

fn dot_product(vec1: &(f64, f64), vec2: &(f64, f64)) -> f64 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1
}

fn get_angle_between_vectors(vec1: &(f64, f64), vec2: &(f64, f64)) -> f64 {
    acos(dot_product(vec1, vec2) / (vec1.magnitude() * vec2.magnitude()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add() {
        let a = (1.0, 2.0);
        let b = (2.0, 3.0);

        assert_eq!(add(&a, &b), (3.0, 5.0));
    }

    #[test]
    fn check_subtract() {
        let a = (1.0, 2.0);
        let b = (2.0, 3.0);

        assert_eq!(subtract(&a, &b), (-1.0, -1.0))
    }

    #[test]
    fn check_dot() {
        let a = (2.0, 3.0);
        let b = (6.0, -4.0);
        println!("{}", dot_product(&a, &b));
        assert_eq!(dot_product(&a, &b), 0.0)
    }
}

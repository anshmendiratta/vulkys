use crate::util::type_traits::VectorMethods;
use libm::acos;

type Radians = f64;

/// Treating vectors as mathematical vectors

/// Adding two vectors together term-by-term. Does not account for when the vectors are of different sizes.
fn add(t: &Vec<f64>, j: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(t.len());
    for idx in 0..t.len() {
        result.push(t[idx] + j[idx])
    }
    result
}

/// Similar operation as `add` but subtraction term-by-term.
fn subtract(t: &Vec<f64>, j: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(t.len());
    for idx in 0..t.len() {
        result.push(t[idx] - j[idx])
    }
    result
}

/// Dot product meant for two-dimensional vectors
fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    vec1[0] * vec2[0] + vec1[1] * vec2[1]
}

/// Reverse-enginerring the angle between two vectors using an alternate definition of the dot product.
fn get_angle_between_vectors(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    acos(dot_product(&vec1, &vec2) / (vec1.magnitude() * vec2.magnitude()))
}

// Unit-tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 3.0, 4.0];

        assert_eq!(add(&a, &b), vec![3.0, 5.0, 7.0])
    }

    #[test]
    fn check_subtract() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 3.0, 4.0];

        assert_eq!(subtract(&a, &b), vec![-1.0, -1.0, -1.0])
    }

    #[test]
    fn check_dot() {
        let a = vec![2.0, 3.0];
        let b = vec![6.0, -4.0];
        println!("{}", dot_product(&a, &b));
        assert_eq!(dot_product(&a, &b), 0.0)
    }
}

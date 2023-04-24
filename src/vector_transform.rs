use crate::type_traits::VectorMethods;
use libm::acos;

type Radians = f64;

/// Treating vectors as mathematical vectors

/// Adding two vectors together term-by-term. Does not account for when the vectors are of different sizes.
fn add(T: &Vec<f64>, J: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(T.len());
    for idx in 0..T.len() {
        result.push(T[idx] + J[idx])
    }
    result
}

/// Similar operation as `add` but subtraction term-by-term.
fn subtract(T: &Vec<f64>, J: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(T.len());
    for idx in 0..T.len() {
        result.push(T[idx] - J[idx])
    }
    result
}

/// Dot product meant for two-dimensional vectors
fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    vec1[0] * vec2[0] + vec1[1] * vec2[1]
}

/// Reverse-enginerring the angle between two vectors using an alternate definition of the dot product.
fn get_angle_between_vectors(vec1: Vec<f64>, vec2: Vec<f64>) -> f64 {
    acos(dot_product(&vec1, &vec2) / (vec1.magnitude() * vec2.magnitude()))
}

// Unit-tests
// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn check_rotate_vector() {
//     assert_eq!(rotate_vector([1, 1].to_vec(), consts::PI), [0, 1].to_vec())
// }

// #[test]
// fn check_angle_between() {
//     let vec1 = vec![0.0, 1.0];
//     let vec2 = vec![1.0, 0.0];
//     assert_eq!::<bool>(get_angle_between_vectors(&vec1, &vec2), std::f64::consts::PI / 2.0)
// }

//     #[test]
//     fn check_rotation() {
//         let mut vector: Vec<f64> = vec![1.0, 0.0];
//         // let equality: bool = eq(rotate_vector(&mut vector, std::f64::consts::PI / 4.0), &vec![1.0/2_f64.powf(0.5), 1.0/2_f64.powf(0.5)]);
//         // println!("{}", equality)
//         assert_eq!(rotate_vector(&mut vector, std::f64::consts::PI / 4.0), vec![1.0/2_f64.powf(0.5), 1.0/2_f64.powf(0.5)])
//     }
// }

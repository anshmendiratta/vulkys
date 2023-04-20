use libm::{acos, atan2};
// use std::f64::consts;
use crate::type_traits::VectorMethods;

// fn eq(one: &Vec<f64>, other: &Vec<f64>) -> bool {
//     let mut counter: usize = 0;

//     if &one.len() != &other.len() {
//         return false
//     } else {
//         while &counter <= &one.len() {
//             let mut idx: usize = 0;
//             if &one[idx] == &other[idx] {
//                 counter += 1
//             }
//             idx += 1
//         }
//     }
//     counter == one.len()
// }

type Radians = f64;

fn add(T: &Vec<f64>, J: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(T.len());
    for idx in 0..T.len() {
        result.push(T[idx] + J[idx])
    }
    result
}

fn subtract(T: &Vec<f64>, J: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(T.len());
    for idx in 0..T.len() {
        result.push(T[idx] - J[idx])
    }
    result
}

fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    vec1[0] * vec2[0] + vec1[1] * vec2[1]
}

fn get_angle_between_vectors(vec1: Vec<f64>, vec2: Vec<f64>) -> f64 {
    acos(dot_product(&vec1, &vec2) / (vec1.magnitude() * vec2.magnitude()))
}

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

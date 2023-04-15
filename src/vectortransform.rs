use libm::{atan2, acos};
// use std::f64::consts;

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

fn get_angle_between_vectors(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    acos(dot_product(vec1, vec2) / (vec1.norm() * vec2.norm()))
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


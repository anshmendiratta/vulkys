use libm::{atan2, acos};
// use std::f64::consts;

// trait Norm {
//     fn norm(&self) -> f64 {
//         0.0
//     }
// }

// impl Norm for Vec<f64> {
//     fn norm(&self) -> f64 {
//         let mut square_sum: f64 = 0.0;
//         for basis in *&self {
//             square_sum += basis.powf(2.0)
//         }

//         square_sum.powf(0.5)
//     }
// }

// impl PartialEq for Vec<f64> {
//     fn eq(&self, other: &Vec<f64>) -> bool {
//         let mut counter: usize = 0;
        
//         if &self.len() != &other.len() {
//             false
//         } else {
//             for idx in &self.len().into() {
//                 if &self[idx] == other[idx] {
//                     counter += 1
//                 }
//             };
//             &counter == &self.len()
//         };
//     }
// }

fn eq(one: &Vec<f64>, other: &Vec<f64>) -> bool {
    let mut counter: usize = 0;
    
    if &one.len() != &other.len() {
        return false
    } else {
        while &counter <= &one.len() {
            let mut idx: usize = 0;
            if &one[idx] == &other[idx] {
                counter += 1
            }
            idx += 1
        }
    }
    counter == one.len()
}

fn rotate_vector(vector: &mut Vec<f64>, angle: f64) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0, 0.0];
    
    result[0] = vector[0] * angle.cos() - vector[1] * angle.sin();
    result[1] = vector[0] * angle.sin() + vector[1] * angle.cos();
    
    result
}

fn scale_vector(vector: &Vec<f64>, scalar: f64) -> &Vec<f64> {
    [scalar * vector[0], scalar * vector[1]].to_vec()
}

fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    vec1[0] * vec2[0] + vec1[1] * vec2[1]
}

fn get_angle_between_vectors(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    acos(dot_product(vec1, vec2) / (vec1.norm() * vec2.norm()))
}
  
#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn check_rotation() {
        let mut vector: Vec<f64> = vec![1.0, 0.0];
        // let equality: bool = eq(rotate_vector(&mut vector, std::f64::consts::PI / 4.0), &vec![1.0/2_f64.powf(0.5), 1.0/2_f64.powf(0.5)]);
        // println!("{}", equality)
        assert_eq!(rotate_vector(&mut vector, std::f64::consts::PI / 4.0), vec![1.0/2_f64.powf(0.5), 1.0/2_f64.powf(0.5)])
    }
}


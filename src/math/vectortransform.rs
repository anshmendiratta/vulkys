use std::f64::consts;

fn rotate_vector(vector: Vec<f32>, angle: f32) -> Vec<f32> {
    vector[0] = vector[0]*angle.cos() - y*angle.sin();
    vector[0] = vector[0]*angle.sin() + y*angle.cos();
    
    vector
}

fn scale_vector(vector: Vec<f32>, scalar: f32) -> Vec<f32> {
    [vector[0]*scalar, vector[1]*scalar].to_vec()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn check_rotate_vector() {
//         assert_eq!(rotate_vector([1, 1].to_vec(), consts::PI), [0, 1].to_vec())    
//     }
    
// }

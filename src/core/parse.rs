use serde_json::Value;

use crate::{
    physics::{circle::Circle, rigidbody::RigidBody},
    FVec2,
};

pub fn parse_serde_value(json: Value) -> Result<Vec<RigidBody>, std::io::Error> {
    let objects_arr = &json;
    let mut objects: Vec<RigidBody> = Vec::new();

    if let Some(objects_arr) = objects_arr.as_object() {
        for object in objects_arr.into_iter() {
            let inner_objects = object.1;
            for object_struct in inner_objects.as_array().unwrap() {
                let object_types: Vec<&String> =
                    object_struct.as_object().unwrap().keys().collect();
                object_types.iter().for_each(|k| {
                    let Some(rigidbody) =
                        rigidbody_from_value(k, object_struct[k].as_array().unwrap())
                    else {
                        return;
                    };
                    objects.push(rigidbody);
                })
            }
        }
    }

    Ok(objects)
}

fn rigidbody_from_value(object_type: &String, json: &Vec<Value>) -> Option<RigidBody> {
    let rbid = match json[1].clone().as_number() {
        Some(num) => num.as_u64().unwrap() as u8,
        _ => return None,
    };
    let json = json[0].clone();
    match &object_type[..] {
        "Circle_" => {
            let position_object = json["position"].as_object().unwrap();
            let (x, y) = (
                position_object["x"].as_number().unwrap(),
                position_object["y"].as_number().unwrap(),
            );
            let position: FVec2 =
                FVec2::new(x.as_f64().unwrap() as f32, y.as_f64().unwrap() as f32);

            let velocity_object = json["velocity"].as_object().unwrap();
            let (x, y) = (
                velocity_object["x"].as_number().unwrap(),
                velocity_object["y"].as_number().unwrap(),
            );
            let velocity: FVec2 =
                FVec2::new(x.as_f64().unwrap() as f32, y.as_f64().unwrap() as f32);

            let radius = json["radius"].as_number().unwrap().as_f64().unwrap() as f32;

            let circle = Circle {
                position,
                velocity,
                radius,
            };
            return Some(RigidBody::Circle_(circle, rbid));
        }
        _ => None,
    }
}

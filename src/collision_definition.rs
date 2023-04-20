use crate::resolve_collisions;
use crate::rigidbodies::{RigidBody, Updateable, HandleData};
use crate::world::*;

#[derive(Debug)]
pub enum CollisionType<T: Updateable + HandleData<T>> {
    ObjObj(T, T),
    ObjWorld(T, World<T>),
}

#[derive(Debug)]
pub struct Collision<T: Updateable + HandleData<T>> {
    objects: CollisionType<T>,
    time: f64,
}

impl<T: Updateable + HandleData<T>> CollisionType<T> {
    fn get_participants(&self) -> (T, Option<T>) {
        match self {
            CollisionType::ObjObj(body1, body2) => (*body1, Some(*body2)),
            CollisionType::ObjWorld(body1, world) => (*body1, None),
        }
    }
}

impl<T: Updateable + HandleData<T>> Collision<T> {
    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_objects(&self) -> (T, Option<T>) {
        self.objects.get_participants()
    }
}

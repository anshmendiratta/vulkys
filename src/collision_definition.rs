
use crate::rigidbodies::{Updateable};
use crate::world::*;
pub struct Collision<T>
where
    T: Updateable,
{
    objects: CollisionType<T>,
    time: f64,
}

pub enum CollisionType<T>
where
    T: Updateable,
{
    ObjObj(T, T),
    ObjWorld(T, World),
}

impl<T> CollisionType<T>
where
    T: Updateable + AsRef<T>,
{
    fn get_participants(&self) -> (&T, Option<&T>) {
        match self {
            CollisionType::ObjObj(body1, body2) => (body1, Some(body2)),
            CollisionType::ObjWorld(body1, _world) => (body1, None),
        }
    }
}

impl<T> Collision<T>
where
    T: Updateable + AsRef<T>,
{
    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_objects(&self) -> (&T, Option<&T>) {
        self.objects.get_participants()
    }
}

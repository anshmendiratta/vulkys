
use crate::rigidbodies::{HandleData, Updateable};
use crate::world::*;

pub enum CollisionType<T>
where
    T: Updateable + HandleData<T>,
{
    ObjObj(T, T),
    ObjWorld(T, World),
}

pub struct Collision<T>
where
    T: Updateable + HandleData<T>,
{
    objects: CollisionType<T>,
    time: f64,
}

impl<T> CollisionType<T>
where
    T: Updateable + HandleData<T> + AsRef<T>,
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
    T: Updateable + HandleData<T> + AsRef<T>,
{
    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_objects(&self) -> (&T, Option<&T>) {
        self.objects.get_participants()
    }
}

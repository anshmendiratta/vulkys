
use crate::rigidbodies::{Updateable};
use crate::world::*;

/// Defining the Collision struct between two objects of type T and the time at which it occurs.
pub struct Collision<T>
where
    T: Updateable,
{
    objects: CollisionType<T>,
    time: f64,
}

/// A useful enumeration when dealing with resolving collisions. ObjWorld collisions are much simpler to handle than ObjObj collisions.
pub enum CollisionType<T>
where
    T: Updateable,
{
    ObjObj(T, T),
    ObjWorld(T, World),
}

/// The implementation of CollisionType<T> defining a method that can be used as `Collision.objects.get_particapnts()` to get all offending bodies in a collision
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

/// Getters for Collision. Uses `CollisionType`'s method mentioned above.
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

use crate::physics::motion::physics;
use crate::physics::world::*;

pub fn step_world(world: World) {
    let dt: f64 = world.get_timestep();

    for mut object in world.get_objects() {
        physics::update_velocity(&mut *object, &dt);
        physics::update_position(&mut *object, &dt);
    }
}

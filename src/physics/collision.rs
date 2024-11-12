use super::rigidbody::GenericObject;

struct CollisionHandler {
    collision_type: CollisionObjectType,
    other: dyn GenericObject,
}

enum CollisionObjectType {
    World,
    Object,
}

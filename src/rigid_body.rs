use crate::vector3::Vector3;

pub struct RigidBody {
    mass: f32,
    radius: f32,
    pos: Vector3,
    velocity: Vector3,
}
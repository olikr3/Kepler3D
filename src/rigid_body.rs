use crate::vector3::Vector3;

pub struct RigidBody {
    mass: f32,
    radius: f32,
    pos: Vector3,
    velocity: Vector3,
    //force: Vector3,
    //torque: Vector3,
    //shape: Shape -- for the future...
}

impl RigidBody {

    pub fn new(mass: f32, radius:f32, pos: Vector3, velocity: Vector3) -> Self {
        Self { mass, radius, pos, velocity }
    }

    fn compute_force_and_torque(&self) {
        todo!()
    }

    fn accelerate(acc: Vector3) {
        todo!()
    }
}
use crate::vector3::Vector3;
use crate::quaternion::Quaternion;
use crate::matrix::Matrix;

/// linear velocity:  x(t) -> rate of change of position
/// angular velocity: w(t) -> rate of change of orientation

pub struct RigidBody {

    //primary
    orientation: Quaternion,
    angularMomentum: Vector3,

    // constants
    mass: f32,
    inertia: Matrix,
    inverse_intertia: Matrix,

    // state variables
    pos: Vector3,
    velocity: Vector3,
    q: Quaternion,
    //force: Vector3,
    //torque: Vector3,
    //shape: Shape -- for the future...
}

impl RigidBody {

    pub fn new() -> Self {
        todo!()
    }

    fn compute_force_and_torque(&self) {
        todo!()
    }

    fn accelerate(acc: Vector3) {
        todo!()
    }
}
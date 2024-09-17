use crate::vector3::Vector3;
use crate::quaternion::Quaternion;
use crate::matrix::Matrix;

/// linear velocity:  x(t) -> rate of change of position
/// angular velocity: w(t) -> rate of change of orientation

pub struct RigidBody {

    //primary
    orientation: Quaternion,
    angular_momentum: Vector3,

    // secondary
    spin: Quaternion,
    angular_velocity: Vector3,

    // constants
    inertia:f32,
    inverse_inertia: f32
}

impl RigidBody {

    pub fn new() -> Self {
        todo!()
    }

    /// update state of rigid body
    pub fn recalculate(&mut self) {
        self.angular_velocity = self.angular_momentum * self.inverse_inertia;
        self.orientation.normalize();
        let q = Quaternion::new(0.0, self.angular_velocity.x(), self.angular_velocity.y(), self.angular_velocity.z());
        self.spin = 0.5 * q * self.orientation;
    }

}
use nalgebra::{Vector3, Matrix3, Quaternion, UnitQuaternion};

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vector3<f32>,
    pub orientation: UnitQuaternion<f32>,
    pub linear_velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub mass: f32,
    pub inverse_mass: f32,
    pub inertia_tensor: Matrix3<f32>,
    pub inverse_inertia_tensor: Matrix3<f32>,
}

impl RigidBody {
    
    pub fn new(
        position: Vector3<f32>,
        orientation: Quaternion<f32>,
        mass: f32,
        inertia_tensor: Matrix3<f32>,
    ) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let inverse_inertia_tensor = if mass > 0.0 {
            inertia_tensor.try_inverse().unwrap_or_else(Matrix3::zeros)
        } else {
            Matrix3::zeros()
        };

        RigidBody {
            position,
            orientation: UnitQuaternion::from_quaternion(orientation),
            linear_velocity: Vector3::zeros(),
            angular_velocity: Vector3::zeros(),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
        }
    }

    pub fn world_inverse_inertia_tensor(&self) -> Matrix3<f32> {
        let rotation = self.orientation.to_rotation_matrix();
        rotation * self.inverse_inertia_tensor * rotation.transpose()
    }

    pub fn apply_force(&mut self, force: Vector3<f32>, point: Vector3<f32>) {
        self.linear_velocity += force * self.inverse_mass;

        // compute torque
        let r = point - self.position;
        let torque = r.cross(&force);
        self.angular_velocity += self.world_inverse_inertia_tensor() * torque;
    }

    /// Integrates the rigid body's state over time step `dt`.
    pub fn integrate(&mut self, dt: f32) {
        self.position += self.linear_velocity * dt;

        let angular_displacement =
            UnitQuaternion::from_scaled_axis(self.angular_velocity * dt);
        self.orientation = (angular_displacement * self.orientation).normalize();
    }
}
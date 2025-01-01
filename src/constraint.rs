use nalgebra::Vector3;
use crate::RigidBody;

pub trait Constraint {
    fn apply(&self, dt: f32, bodies: &mut [RigidBody]);
}

pub struct FixedPointConstraint {
    body_index: usize,
    fixed_point: Vector3<f32>,
}

impl Constraint for FixedPointConstraint {
    fn apply(&self, _dt: f32, bodies: &mut [RigidBody]) {
        let body = &mut bodies[self.body_index];
        body.position = self.fixed_point; // Overwrite the position to the fixed point
        body.linear_velocity = Vector3::zeros(); // Stop motion
        body.angular_velocity = Vector3::zeros(); // Stop rotation
    }
}

pub struct DistanceConstraint {
    body_a: usize,
    body_b: usize,
    target_distance: f32,
}

impl Constraint for DistanceConstraint {
    fn apply(&self, _dt: f32, bodies: &mut [RigidBody]) {
        let body_a = &mut bodies[self.body_a];
        let body_b = &mut bodies[self.body_b];

        let delta = body_b.position - body_a.position;
        let current_distance = delta.norm();
        let correction = (self.target_distance - current_distance) * 0.5;

        if current_distance > 0.0 {
            let correction_vector = delta.normalize() * correction;
            body_a.position -= correction_vector; // Move body A
            body_b.position += correction_vector; // Move body B
        }
    }
}



pub trait Constraint {
    /// checks whether the constraint is satisfied.
    fn evaluate(&self) -> f32;

    /// applies the necessary forces to maintain the constraint.
    fn solve(&mut self);
}

/// represents a distance constraint between two points on two bodies.
pub struct DistanceConstraint {
    pub point_a: (f32, f32, f32),
    pub point_b: (f32, f32, f32),
    pub rest_distance: f32,
}

impl Constraint for DistanceConstraint {
    fn evaluate(&self) -> f32 {
        let (ax, ay, az) = self.point_a;
        let (bx, by, bz) = self.point_b;
        let distance = ((bx - ax).powi(2) + (by - ay).powi(2) + (bz - az).powi(2)).sqrt();
        return distance - self.rest_distance;
    }

    fn solve(&mut self) {
        todo!();
    }
}


use crate::rigid_body::RigidBody;
use crate::constraint::Constraint;


/// a collection of rigid bodies and constraints
pub struct PhysicsWorld {
    objects: Vec<RigidBody>,
    constraints: Vec<Box<dyn Constraint>>,
}

impl PhysicsWorld {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.objects.push(body);
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.integrate(dt);
        }

        for constraint in &self.constraints {
            constraint.apply(dt, &mut self.bodies);
        }
    }

    // simulate the world for 'dt' seconds
    pub fn simulate(&mut self, dt: usize) {
        for _ in 0..dt {
            todo!()
            //something like:
            //apply gravity
            //solve constraints etc.
        }
    }
}
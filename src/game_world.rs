use crate::rigid_body::RigidBody;
use crate::constraint::Constraint;

pub struct GameWorld {
    objects: Vec<RigidBody>,
    constraints: Vec<Box<dyn Constraint>>,
    gravity: f64,
}

impl GameWorld {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            gravity: 9.81,
            constraints: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.objects.push(body);
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
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

    pub fn disable_gravity(&mut self) {
        self.gravity = 0.0;
    }
}
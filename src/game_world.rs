mod rigid_body;

pub struct GameWorld {
    objects: Vec<RigidBody>,
}

impl GameWorld {
    
    fn new() -> Self {
        Self { {Vec::new()} }
    }
}

mod rigid_body;

pub struct GameWorld {
    objects: Vec<RigidBody>,
    gravity: f64,
}

impl GameWorld {

    fn new() -> Self {
        Self {
            objects: Vec::new(),
            gravity: 9.81,
        }
    }

    fn add_body(&mut self, body: RigidBody) {
        self.objects.push(body);
    }

    // simulate the world for 'dt' seconds
    fn simulate(&mut self, dt: usize) {
        for _ in 0..dt {
            todo!()
        }
    }

    fn disable_gravity(&mut self) {
        self.gravity = 0.0;
    }
}
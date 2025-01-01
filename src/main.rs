mod rigid_body;
mod physics_world;
mod constraint;

use crate::physics_world::PhysicsWorld;
use crate::rigid_body::RigidBody;

const NUM_BODIES: usize = 100;

fn main() {
    
    let mut bodies: Vec<RigidBody> = vec![];
/*
    for i in 0..NUM_BODIES {

        let pos_vec = Vector3::new(0.0 + i as f32, 0.0 + i as f32, 0.0 + i as f32);
        let vel_vec = Vector3::new(0.0 + i as f32, 0.0 + i as f32, 0.0 + i as f32);
        
        // Create a RigidBody with mass and inertia, and add to bodies vector
        let body = RigidBody::new(100.0, 100.0, pos_vec, vel_vec);
        bodies.push(body);

    }
*/

    let mut system = PhysicsWorld::new();
    println!("Hello, world!");
}
